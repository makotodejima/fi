use crate::schema::*;
use crate::Snapshot;
use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::value::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use termion::color;

#[derive(Queryable, Insertable)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub description: String,
}

impl Account {
    pub fn sync(conn: &PgConnection, res: Value) {
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

                let result = Account::add_new_account(
                    conn,
                    account_id.to_owned(),
                    account_name.to_owned(),
                    account_currency,
                    account_type,
                );

                match result {
                    Ok(new_account) => println!(
                        "✅  {}Added new accounts: {}{}",
                        color::Fg(color::Cyan),
                        new_account.name,
                        color::Fg(color::Reset),
                    ),
                    Err(err) => println!(
                        "Skipping inserting '{}'. Maybe already there? Error: {}",
                        account_name, err
                    ),
                }

                Snapshot::update_snapshots(&conn, account_id.to_owned(), amounts_by_date);
            } else {
                panic!("Failed to find rows in notion_table");
            }
        }
    }

    pub fn add_new_account(
        conn: &PgConnection,
        id: String,
        name: String,
        currency: String,
        description: String,
    ) -> Result<Account, diesel::result::Error> {
        use accounts::id as id_column;

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
            .get_result::<Account>(conn)
    }

    pub fn display_accounts(conn: &PgConnection) {
        let all_accounts = accounts::table
            .load::<Account>(conn)
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

    pub fn delete_accounts(conn: &PgConnection) {
        use diesel::delete;
        delete(accounts::table)
            .execute(conn)
            .expect("Error deleting accounts table");
    }
}
