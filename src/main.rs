#[macro_use] extern crate rocket;

mod schema;
mod models;
mod repositories;

use rocket::serde::json::{Value, json, Json};
use rocket::response::status;
use rocket_sync_db_pools::database;
use models::{Rustacean, NewRustacean};

use crate::repositories::RustaceanRepository;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(db: DbConn) -> Value {
    db.run(|c| {
        let rustaceans = RustaceanRepository::find_multiple(c, 100)
            .expect("DB error");
        json!(rustaceans)
    }).await
}

#[get("/rustaceans/<id>")]
async fn get_rustacean(id: i32, db: DbConn) -> Value {
    db.run(move |c| {
        let rustacean = RustaceanRepository::find(c, id)
            .expect("DB error");
        json!(rustacean)
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = RustaceanRepository::create(c, new_rustacean.into_inner())
            .expect("DB error when inserting");
        json!(result)
    }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(db: DbConn, id: i32, rustacean: Json<Rustacean>) -> Value {
    db.run(move |c| {
        let result = RustaceanRepository::save(c, id, rustacean.into_inner())
            .expect("DB error when updating");
        json!(result)
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(db: DbConn, id: i32) -> status::NoContent {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .expect("DB error when deleting");
        status::NoContent
    }).await
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
