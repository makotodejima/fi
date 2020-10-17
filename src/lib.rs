#[macro_use]
extern crate diesel;

pub mod account;
pub mod cli;
pub mod currency;
pub mod schema;
pub mod snapshot;

use account::{delete_accounts, Account};
use chrono::NaiveDate;
use currency::Currency;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use reqwest;
use schema::*;
use serde_json::value::Value;
use snapshot::{delete_snapshots, Snapshot};
use termion::color;
use textplots::{Chart, Plot, Shape};

pub fn display_latest_sum(conn: &PgConnection, currency: &Currency) {
    let table: Vec<(Snapshot, Account)> = snapshots::table
        .distinct_on(snapshots::account_id)
        .inner_join(accounts::table)
        .filter(accounts::currency.eq(&currency.as_str()))
        .order((snapshots::account_id, snapshots::date.desc()))
        .load(conn)
        .expect("Error loading latest sum");

    let mut sum = 0;
    println!("{} - latest", currency.as_str());
    println!("---");
    for (snapshot, account) in table {
        println!("{}: {} {}", snapshot.date, account.name, snapshot.amount);
        sum += snapshot.amount;
    }
    println!("---");
    println!("Total: {}", sum);
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

    println!("Net worth in {}\n===", currency.as_str());
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

    println!("---");
    println!("Total: {}", sum);
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

            println!(
                "---\n{} accounts ({} {} = 1.00)",
                currency.as_str(),
                rate,
                currency.as_str(),
            );

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
    delete_snapshots(conn);
    delete_accounts(conn);
}
