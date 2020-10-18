use crate::schema::*;
use chrono::NaiveDate;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::PgConnection;
use std::collections::HashMap;
use termion::color;

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
        let result = diesel::insert_into(snapshots::table)
            .values(&new_snapshot)
            .on_conflict(on_constraint("snapshot_unique"))
            .do_nothing()
            .get_result::<Snapshot>(conn);

        match result {
            Ok(new_snapshot) => println!(
                "{}Added new snapshot: for {}{}",
                color::Fg(color::Cyan),
                new_snapshot.date,
                color::Fg(color::Reset),
            ),
            Err(err) => println!(
                   "Skipping inserting snapshot with id '{}', date '{}'.\nMaybe already there? Error: {}",
                   new_snapshot.account_id,
                   new_snapshot.date,
                   err
               )
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

    pub fn display_snapshots(conn: &PgConnection) {
        let all_snapshots = snapshots::table
            .load::<Snapshot>(conn)
            .expect("Error getting snapshots");
        println!("\n *Displaying all snapshots");
        println!("---");
        for snapshot in all_snapshots {
            println!("Id: {}", snapshot.id);
            println!("Account: {}", snapshot.account_id);
            println!("Date: {}", snapshot.date);
            println!("Amount: {}", snapshot.amount);
            println!("---");
        }
    }

    pub fn delete_snapshots(conn: &PgConnection) {
        use diesel::delete;
        delete(snapshots::table)
            .execute(conn)
            .expect("Error deleting snapshots table");
    }
}
