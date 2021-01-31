#[macro_use]
extern crate diesel;

mod account;
pub mod cli;
mod currency;
mod schema;
mod snapshot;

use account::Account;
use chrono::NaiveDate;
use cli::Cli;
use currency::Currency;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use reqwest;
use schema::*;
use serde_json::value::Value;
use snapshot::Snapshot;
use std::env;
use std::error::Error;
use termion::color;
use textplots::{Chart, Plot, Shape};

pub fn run(database_url: &str, command: Cli) -> Result<(), Box<dyn Error>> {
    let conn = PgConnection::establish(&database_url).expect("Error connecting to the database");
    match command {
        Cli::Pull { currency } => {
            if currency == "all".to_string() {
                sync_all(&conn).expect("Error occurred while synching");
            }
            let given_currency = Currency::from_str(&currency);
            let notion_api_url = get_notion_api_url(&given_currency);
            let res = reqwest::blocking::get(&notion_api_url)?.json::<Value>()?;
            Account::sync(&conn, res);
        }
        Cli::History { currency } => {
            let given_currency = Currency::from_str(&currency);
            display_history(&conn, &given_currency);
        }
        Cli::Sum { currency } => {
            let given_currency = Currency::from_str(&currency);
            display_latest_sum(&conn, &given_currency);
        }
        Cli::NetWorth { currency } => {
            let given_currency = Currency::from_str(&currency);
            display_net_worth(&conn, &given_currency);
        }
        Cli::Delete => {
            delete_data(&conn);
        }
    }
    Ok(())
}

fn get_notion_api_url(currency: &Currency) -> String {
    let mut notion_api_url = String::from("NOTION_API_URL_");
    notion_api_url.push_str(currency.as_str());
    match env::var(notion_api_url) {
        Ok(url) => url,
        Err(err) => panic!("Failed to get notion api url. Error: {}", err),
    }
}

fn sync_all(conn: &PgConnection) -> Result<(), Box<dyn Error>> {
    use std::process::exit;
    let currencies = [Currency::EUR, Currency::JPY, Currency::USD];
    for cur in &currencies {
        let notion_api_url = get_notion_api_url(cur);
        let res = reqwest::blocking::get(&notion_api_url)?.json::<Value>()?;
        Account::sync(&conn, res);
    }
    println!("Synching all accounts and snapshots completed. \n");
    exit(0);
}

pub fn display_latest_sum(conn: &PgConnection, currency: &Currency) {
    let table: Vec<(Snapshot, Account)> = snapshots::table
        .distinct_on(snapshots::account_id)
        .inner_join(accounts::table)
        .filter(accounts::currency.eq(&currency.as_str()))
        .order((snapshots::account_id, snapshots::date.desc()))
        .load(conn)
        .expect("Error loading latest sum");

    let mut sum = 0;
    println!("\n{} - latest", currency.as_str());
    println!("---");
    for (snapshot, account) in table {
        println!("{}: {} {}", snapshot.date, account.name, snapshot.amount);
        sum += snapshot.amount;
    }
    println!("---");
    println!("Total: {}\n", sum);
}

pub fn display_history(conn: &PgConnection, currency: &Currency) {
    let table: Vec<(NaiveDate, Option<i64>)> = snapshots::table
        .inner_join(accounts::table)
        .select((snapshots::date, sql("sum(amount)")))
        .filter(accounts::currency.eq(&currency.as_str()))
        .group_by(snapshots::date)
        .order(snapshots::date.asc())
        .load(conn)
        .expect("Error loading table");

    println!("\n{} - history", &currency.as_str());
    println!("---");

    let mut prev: Option<i64> = None;
    for (date, sum) in &table {
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

    let points: Vec<(f32, f32)> = table
        .to_owned()
        .into_iter()
        .enumerate()
        .map(|(idx, (_, sum))| (idx as f32, sum.unwrap() as f32))
        .collect();

    Chart::new(100, 40, 0.0, *&table.len() as f32 - 1.0)
        .lineplot(&Shape::Lines(&points))
        .nice();
    println!("\n");
}

pub fn display_net_worth(conn: &PgConnection, currency: &Currency) {
    let join = snapshots::table
        .distinct_on(snapshots::account_id)
        .inner_join(accounts::table)
        .order((snapshots::account_id, snapshots::date.desc()));

    let base_currency_table: Vec<(Snapshot, Account)> = join
        .filter(accounts::currency.eq(currency.as_str()))
        .load(conn)
        .expect("Error loading latest sum");

    let mut sum = 0;

    println!("\nNet worth in {}\n===", currency.as_str());
    println!("{} accounts", currency.as_str());
    for (snapshot, account) in base_currency_table {
        println!("{}: {} {}", snapshot.date, account.name, snapshot.amount);
        sum += snapshot.amount;
    }
    let mut exchange_rate_api_url = String::from("https://api.exchangeratesapi.io/latest?base=");
    exchange_rate_api_url.push_str(&currency.as_str());
    let req = reqwest::blocking::get(&exchange_rate_api_url);

    match req {
        Ok(res) => {
            let res_json = res.json::<Value>().unwrap();
            for cur in &Currency::the_others(currency) {
                display_latest_converted(conn, &res_json, cur, &mut sum);
            }
        }
        Err(err) => panic!(err),
    }

    println!("===");
    println!("Total: {} {}", sum, &currency.as_str());
    println!("\n");
}

fn display_latest_converted(
    conn: &PgConnection,
    exchange_rate_json: &Value,
    currency: &Currency,
    sum: &mut i32,
) {
    if let Value::Number(rate) = &exchange_rate_json["rates"][currency.as_str()] {
        if let Some(rate) = rate.as_f64() {
            let table: Vec<(Snapshot, Account)> = snapshots::table
                .distinct_on(snapshots::account_id)
                .inner_join(accounts::table)
                .order((snapshots::account_id, snapshots::date.desc()))
                .filter(accounts::currency.eq(currency.as_str()))
                .load(conn)
                .expect("Error loading latest converted sum");

            if let Value::String(base) = &exchange_rate_json["base"] {
                println!(
                    "---\n{} accounts (1.00 {} = {} {})",
                    currency.as_str(),
                    base,
                    rate,
                    currency.as_str(),
                );
            } else {
                panic!("Error resolving base currency");
            }

            for (snapshot, account) in table {
                let converted_amount = snapshot.amount as f64 / rate;
                println!(
                    "{}: {} {:.0}",
                    snapshot.date, account.name, converted_amount
                );
                *sum += converted_amount as i32;
            }
        }
    }
}

pub fn delete_data(conn: &PgConnection) {
    Snapshot::delete_snapshots(conn);
    Account::delete_accounts(conn);
    println!("Deleted all rows in tables");
}
