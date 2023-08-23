use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};
use crate::schema::rustaceans;

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Rustacean {
    // PRIMARY KEY can't be NULLABLE, if NULLABLE in schema change that
    // NULLABLE in schema must be Option<T> in struct
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}