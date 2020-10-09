extern crate diesel_demo;
extern crate dotenv;
extern crate reqwest;
extern crate serde_json;

use diesel_demo::DieselDemo;
use dotenv::dotenv;
use serde_json::value::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading database url");
    let demo_db = DieselDemo::new(database_url);
    let notion_api_url = env::var("NOTION_API_URL").expect("Error loading notion api url");
    let res = reqwest::blocking::get(&notion_api_url)?.json::<Value>()?;

    let notion_table: Vec<Value>;
    if let Value::Array(vec) = res {
        notion_table = vec;
    } else {
        panic!("Error: Received non vector response");
    };

    for notion_row in notion_table {
        if let Value::Object(row) = notion_row {
            let mut account_id = String::new();
            let mut account_type = String::new();
            let mut account_name = String::new();
            let mut account_currency = String::new();
            let mut amounts_by_date = HashMap::new();
            for (key, val) in row {
                match key.as_str() {
                    "id" => {
                        account_id = val.as_str().unwrap().to_string();
                    }
                    "Type" => {
                        account_type = val.as_str().unwrap().to_string();
                    }
                    "Name" => {
                        account_name = val.as_str().unwrap().to_string();
                    }
                    "Currency" => {
                        account_currency = val.as_str().unwrap().to_string();
                    }
                    _ => {
                        if let Value::Number(num) = val {
                            let num_as_i64 = num.as_i64().unwrap();
                            let amount = i32::try_from(num_as_i64)
                                .expect("Error: Failed to convert from i64");
                            amounts_by_date.insert(key, amount);
                        } else {
                            println!("Error: Unknown type of key found");
                        }
                    }
                }
            }

            let result = demo_db.add_new_account(
                account_id.to_owned(),
                account_name.to_owned(),
                account_currency,
                account_type,
            );

            match result {
                Ok(new_account) => println!("Added new accounts: {}", new_account.name),
                Err(err) => println!(
                    "Failed to insert '{}'. Maybe already there? Error: {}",
                    account_name, err
                ),
            }

            demo_db.update_snapshots(account_id.to_owned(), amounts_by_date)
        } else {
            println!("Failed to find rows.");
        }
    }
    demo_db.run();
    Ok(())
}
