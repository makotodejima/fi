use dotenv::dotenv;
use fi::account::sync;
use fi::cli::Cli;
use fi::DieselConn;
use reqwest;
use serde_json::value::Value;
use std::env;
use structopt::StructOpt;

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading database url");
    let diesel_conn = DieselConn::new(database_url);
    let args = Cli::from_args();
    match args {
        Cli::Pull { currency } => {
            let notion_api_url = get_notion_api_url(&currency);
            let res = reqwest::blocking::get(&notion_api_url)?.json::<Value>()?;
            sync(&diesel_conn, res);
        }
        Cli::History { currency } => {
            diesel_conn.display_history(&currency);
        }
        Cli::Sum { currency } => {
            diesel_conn.display_latest_sum(&currency);
        }
    }
    // diesel_conn.run();
    Ok(())
}

pub fn get_notion_api_url(currency_key: &str) -> String {
    let mut key = String::from("NOTION_API_URL_");
    key.push_str(currency_key.to_uppercase().as_str());
    match env::var(key) {
        Ok(url) => url,
        Err(err) => panic!("Failed to get notion api url. Error: {}", err),
    }
}
