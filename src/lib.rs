#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use diesel::PgConnection;
use models::Account;
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
        // TODO: subcommand to add
        self.add_new_account();
    }

    fn display_accounts(&self) {
        use schema::accounts::dsl::*;

        let all_accounts = accounts
            .load::<Account>(&self.database_connection)
            .expect("Error getting accounts");
        println!("Displaying all accounts");
        println!("---");

        for account in all_accounts {
            println!("Name: {}", account.name);
            println!("Currency: {}", account.currency);
            println!("Description: {}", account.description);
            println!("---");
        }
    }

    fn add_new_account(&self) -> Account {
        use schema::accounts;
        println!("Creating new account...");

        println!("Id?");
        let mut id = String::new();
        stdin().read_line(&mut id).unwrap();

        println!("Account name?");
        let mut name = String::new();
        stdin().read_line(&mut name).unwrap();

        println!("Currency?");
        let mut currency = String::new();
        stdin().read_line(&mut currency).unwrap();

        println!("Enter description for this account");
        let mut description = String::new();
        stdin().read_line(&mut description).unwrap();

        let new_account = Account {
            id: id.trim().parse().unwrap(),
            name: name.trim().parse().unwrap(),
            currency: currency.trim().parse().unwrap(),
            description: description.trim().parse().unwrap(),
        };

        diesel::insert_into(accounts::table)
            .values(&new_account)
            .get_result(&self.database_connection)
            .expect("Error: failed to insert new account.")
    }
}
