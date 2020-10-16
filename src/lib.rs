#[macro_use]
extern crate diesel;

pub mod account;
pub mod cli;
pub mod schema;
pub mod snapshot;

use account::Account;
use chrono::NaiveDate;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use schema::*;
use snapshot::Snapshot;
use termion::color;

// enum Currency {
//     EUR,
//     JPY,
//     USD,
// }

pub struct DieselConn {
    database_connection: PgConnection,
}

impl DieselConn {
    pub fn new(database_url: String) -> Self {
        let database_connection =
            PgConnection::establish(&database_url).expect("Error connecting to the database");
        Self {
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

    pub fn display_month_sum(&self, currency: &str) {
        let test: Vec<(Snapshot, Account)> = snapshots::table
            .inner_join(accounts::table)
            .filter(accounts::currency.eq(currency))
            .filter(snapshots::date.gt(date(now - 30.days())))
            .load(&self.database_connection)
            .expect("Error loading vector");

        let mut sum = 0;
        println!("{}", currency);
        println!("---");
        for (snapshot, account) in test {
            println!("{}: {} {}", snapshot.date, account.name, snapshot.amount);
            sum += snapshot.amount;
        }
        println!("---");
        println!("Total: {}", sum);
    }

    pub fn display_latest_sum(&self, currency: &str) {
        let given_currency = currency.to_uppercase();
        let table: Vec<(Snapshot, Account)> = snapshots::table
            .distinct_on(snapshots::account_id)
            .inner_join(accounts::table)
            .filter(accounts::currency.eq(&given_currency))
            .order((snapshots::account_id, snapshots::date.desc()))
            .load(&self.database_connection)
            .expect("Error loading latest sum");

        let mut sum = 0;
        println!("{} - latest", given_currency);
        println!("---");
        for (snapshot, account) in table {
            println!("{}: {} {}", snapshot.date, account.name, snapshot.amount);
            sum += snapshot.amount;
        }
        println!("---");
        println!("Total: {}", sum);
    }

    pub fn display_history(&self, currency: &str) {
        let given_currency = currency.to_uppercase();
        let table: Vec<(NaiveDate, Option<i64>)> = snapshots::table
            .inner_join(accounts::table)
            .select((snapshots::date, sql("sum(amount)")))
            .filter(accounts::currency.eq(&given_currency))
            .group_by(snapshots::date)
            .order(snapshots::date.asc())
            .load(&self.database_connection)
            .expect("Error loading table");

        println!("{} - history", given_currency);
        println!("---");

        let mut prev: Option<i64> = None;
        for (date, sum) in table {
            if let Some(prev_sum) = prev {
                let is_going_well = sum.unwrap() as f64 >= prev_sum as f64;
                let diff = sum.unwrap() as f64 - prev_sum as f64;
                let diff_percent = sum.unwrap() as f64 / prev_sum as f64;
                if is_going_well {
                    println!(
                        "{}: {} {} +{} / {:.2}%{}",
                        date,
                        sum.unwrap(),
                        color::Fg(color::Cyan),
                        diff,
                        diff_percent * f64::from(100),
                        color::Fg(color::Reset)
                    );
                } else {
                    println!(
                        "{}: {} {} {} / {:.2}%{}",
                        date,
                        sum.unwrap(),
                        color::Fg(color::Red),
                        diff,
                        diff_percent * f64::from(100),
                        color::Fg(color::Reset)
                    );
                }
            } else {
                // First row of record without prev value
                println!("{}: {}", date, sum.unwrap());
            }
            prev = Some(sum.unwrap());
        }
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
}
