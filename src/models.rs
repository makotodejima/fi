use super::schema::{accounts, snapshots};
use diesel::sql_types::{BigInt, Date, Integer};

use chrono::NaiveDate;

#[derive(Queryable, Insertable)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub description: String,
}

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
