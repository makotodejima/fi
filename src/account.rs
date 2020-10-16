use crate::schema::*;
use crate::{DieselConn, Snapshot};
use serde_json::value::Value;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Queryable, Insertable)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub description: String,
}

pub fn sync(conn: &DieselConn, res: Value) {
    let notion_table: Vec<Value>;
    if let Value::Array(vec) = res {
        notion_table = vec;
    } else {
        panic!("Error: Received non vector response from Notion");
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
                            println!("Error: Unknown type of key found in response");
                        }
                    }
                }
            }

            let result = conn.add_new_account(
                account_id.to_owned(),
                account_name.to_owned(),
                account_currency,
                account_type,
            );

            match result {
                Ok(new_account) => println!("Added new accounts: {}", new_account.name),
                Err(err) => println!(
                    "Skipping inserting '{}'. Maybe already there? Error: {}",
                    account_name, err
                ),
            }

            Snapshot::update_snapshots(
                &conn.database_connection,
                account_id.to_owned(),
                amounts_by_date,
            );
        } else {
            panic!("Failed to find rows in notion_table");
        }
    }
}
