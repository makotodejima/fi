use dotenv::dotenv;
use fi::account::sync;
use fi::DieselConn;
use reqwest;
use serde_json::value::Value;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    /// Currency to sync
    currency: String,
}

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading database url");
    let diesel_conn = DieselConn::new(database_url);
    let args = Cli::from_args();
    let notion_api_url = get_notion_api_url(&args.currency);
    let res = reqwest::blocking::get(&notion_api_url)?.json::<Value>()?;
    sync(&diesel_conn, res);

    // diesel_conn.display_month_sum(&args.currency.to_uppercase());
    // diesel_conn.display_latest_sum(&args.currency.to_uppercase());
    // diesel_conn.display_timeline(&args.currency.to_uppercase());

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
