use crate::schema::*;
use chrono::NaiveDate;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::PgConnection;
use std::collections::HashMap;

#[derive(Queryable)]
pub struct Snapshot {
    pub id: i32,
    pub account_id: String,
    pub date: NaiveDate,
    pub amount: i32,
}

#[derive(Insertable)]
#[table_name = "snapshots"]
pub struct NewSnapshot {
    pub account_id: String,
    pub date: NaiveDate,
    pub amount: i32,
}

impl Snapshot {
    pub fn create_new_snapshot(
        conn: &PgConnection,
        account_id: String,
        ymd_string: String,
        amount: i32,
    ) {
        println!("Creating new snapshot...");
        let new_snapshot = NewSnapshot {
            account_id,
            amount,
            date: ymd_string
                .parse::<NaiveDate>()
                .expect("Error: Failed to parse date string"),
        };
        if let Err(err) = diesel::insert_into(snapshots::table)
            .values(&new_snapshot)
            .on_conflict(on_constraint("snapshot_unique"))
            .do_nothing()
            .get_result::<Snapshot>(conn)
        {
            println!(
                "Skipping inserting snapshot with id '{}', date '{}'.\nMaybe already there? Error: {}",
                new_snapshot.account_id,
                new_snapshot.date,
                err
            );
        }
    }

    pub fn update_snapshots(
        conn: &PgConnection,
        account_id: String,
        amounts_by_date: HashMap<String, i32>,
    ) {
        println!(
            "Updating snapshots for {} with notion's table data...",
            &account_id
        );
        for (date, amount) in amounts_by_date {
            Snapshot::create_new_snapshot(conn, account_id.to_owned(), date, amount)
        }
    }
}
