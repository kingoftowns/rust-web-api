#[macro_use] extern crate rocket;

mod schema;
mod models;
mod repositories;

use rocket::http::Status;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status::{self, Custom};
use rocket_sync_db_pools::database;
use models::{Rustacean, NewRustacean};
use repositories::RustaceanRepository;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    }).await
}

#[get("/rustaceans/<id>")]
async fn get_rustacean(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(db: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(db: DbConn, id: i32, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(db: DbConn, id: i32) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
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
