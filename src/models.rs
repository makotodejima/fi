use super::schema::accounts;
extern crate diesel;

use diesel::sql_types::Date;

#[derive(Queryable, Insertable)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub description: String,
}

// #[derive(Insertable)]
// #[table_name = "accounts"]
// pub struct NewAccount {
//     pub name: String,
//     pub currency: String,
//     pub description: String,
// }

// impl NewAccount {
//     pub fn new(name: String, currency: String, description: String) -> Self {
//         Self {
//             name,
//             currency,
//             description,
//         }
//     }
// }

#[derive(Queryable)]
pub struct Snapshot {
    pub id: i32,
    pub account_id: String,
    pub date: Date,
    pub amount: i32,
}

// #[derive(Insertable)]
// #[table_name = "snapshots"]
// pub struct NewSnapshot {
//     pub account_id: String,
//     pub amount: i32,
// }
