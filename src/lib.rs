#[macro_use]
extern crate diesel;

mod models;
mod schema;

use chrono::NaiveDate;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::PgConnection;
use models::{Account, NewSnapshot, Snapshot};
use std::collections::HashMap;
use std::io::{stdin, Read};

pub struct DieselDemo {
    database_connection: PgConnection,
}

impl DieselDemo {
    pub fn new(database_url: String) -> DieselDemo {
        let database_connection =
            PgConnection::establish(&database_url).expect("Error connecting to the database");
        DieselDemo {
            database_connection,
        }
    }

    pub fn run(&self) {
        self.display_accounts();
        self.display_snapshots();
        // TODO: subcommand to add
        // self.add_new_account();
        // self.create_new_snapshot();
    }

    fn display_accounts(&self) {
        use schema::accounts::dsl::*;

        let all_accounts = accounts
            .load::<Account>(&self.database_connection)
            .expect("Error getting accounts");
        println!("\n *Displaying all accounts");
        println!("---");

        for account in all_accounts {
            println!("Name: {}", account.name);
            println!("Currency: {}", account.currency);
            println!("Description: {}", account.description);
            println!("---");
        }
    }

    fn display_snapshots(&self) {
        use schema::snapshots::dsl::*;

        let all_snapshots = snapshots
            .load::<Snapshot>(&self.database_connection)
            .expect("Error getting snapshots");
        println!("\n *Displaying all snapshots");
        println!("---");

        for snapshot in all_snapshots {
            println!("Id: {}", snapshot.id);
            println!("Account: {}", snapshot.account_id);
            println!("Date: {}", snapshot.date);
            println!("Amount: {}", snapshot.amount);
            println!("---");
        }
    }

    pub fn add_new_account(
        &self,
        id: String,
        name: String,
        currency: String,
        description: String,
    ) -> Result<Account, diesel::result::Error> {
        use schema::accounts;
        use schema::accounts::id as id_column;

        println!("Creating new account...");

        let new_account = Account {
            id,
            name,
            currency,
            description,
        };

        diesel::insert_into(accounts::table)
            .values(&new_account)
            .on_conflict(id_column)
            .do_nothing()
            .get_result::<Account>(&self.database_connection)
    }

    pub fn create_new_snapshot(&self, account_id: String, ymd_string: String, amount: i32) {
        use schema::snapshots;

        println!("Creating new snapshot...");

        let new_snapshot = NewSnapshot {
            account_id,
            amount,
            date: ymd_string
                .parse::<NaiveDate>()
                .expect("Error: Failed to parse date string"),
        };

        if let Err(err) = diesel::insert_into(snapshots::table)
            .values(&new_snapshot)
            .on_conflict(on_constraint("snapshot_unique"))
            .do_nothing()
            .get_result::<Snapshot>(&self.database_connection)
        {
            println!(
                "Failed to insert snapshot. Maybe already there? Error: {}",
                err
            );
        }
    }

    pub fn update_snapshots(&self, account_id: String, amounts_by_date: HashMap<String, i32>) {
        println!(
            "Updating snapshots for {} with notion's table data...",
            &account_id
        );
        for (date, amount) in amounts_by_date {
            self.create_new_snapshot(account_id.to_owned(), date, amount);
        }
    }
}
