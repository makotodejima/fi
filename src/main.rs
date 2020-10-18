use diesel::prelude::*;
use dotenv::dotenv;
use fi::account::Account;
use fi::cli::Cli;
use fi::currency::Currency;
use fi::{delete_data, display_history, display_latest_sum, display_net_worth};
use reqwest;
use serde_json::value::Value;
use std::env;
use std::error::Error;
use structopt::StructOpt;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading database url");

    let args = Cli::from_args();

    handle_error(run(&database_url.as_str(), args));
}

fn run(database_url: &str, command: Cli) -> Result<(), Box<dyn Error>> {
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
    println!("Synching all accounts and snapshots completed.");
    exit(0);
}

fn handle_error<T>(res: Result<T, Box<dyn Error>>) -> T {
    match res {
        Ok(x) => x,
        Err(e) => print_error_and_exit(&*e),
    }
}

fn print_error_and_exit(err: &dyn Error) -> ! {
    use std::process::exit;
    eprintln!("An unexpected error occurred: {}", err);
    exit(1);
}
