#[macro_use] extern crate rocket;

mod schema;
mod models;

use diesel::prelude::*;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status;
use rocket_sync_db_pools::database;
use schema::rustaceans;
use models::{Rustacean, NewRustacean};

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(db: DbConn) -> Value {
    db.run(|c| {
        let rustaceans = rustaceans::table.order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
            .expect("DB error");
        json!(rustaceans)
    }).await
}

#[get("/rustaceans/<id>")]
async fn get_rustacean(id: i32, db: DbConn) -> Value {
    db.run(move |c| {
        let rustacean = rustaceans::table.find(id)
            .get_result::<Rustacean>(c)
            .expect("DB error");
        json!(rustacean)
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("DB error when inserting");
        json!(result)
    }).await
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Value {
    json!({
        "id": id,
        "name": "John Doe",
        "age": 30,
        "email": "john@doe.com"
    })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
    

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            get_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean
        ])
        .register("/", catchers![
            not_found
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
