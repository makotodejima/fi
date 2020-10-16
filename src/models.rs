use crate::schema::*;

#[derive(Queryable, Insertable)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub description: String,
}
