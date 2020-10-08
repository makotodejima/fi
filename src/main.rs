extern crate diesel_demo;
extern crate dotenv;
extern crate reqwest;
extern crate serde_json;

use diesel_demo::DieselDemo;
use dotenv::dotenv;
use reqwest::Error;
use serde_json::value::Value;
use std::env;

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading database url");
    let demo_db = DieselDemo::new(database_url);
    demo_db.run();
    // let notion_api_url = env::var("NOTION_API_URL").expect("Error loading notion api url");
    // let res = reqwest::blocking::get(&notion_api_url)?.json::<Value>()?;
    // println!("{:#?}", res);
    Ok(())
}

// CREATE TABLE snapshots (
//   id SERIAL PRIMARY KEY,
//   account_id FOREIGN KEY (accounts id),
//   date DATE,
//   amount Int
// )
