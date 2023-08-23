use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::rustaceans;

#[derive(Serialize, Queryable)]
pub struct Rustacean {
    // Nullable Types in the schema must be Option<T> in the model
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}