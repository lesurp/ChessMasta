#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Json;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use models::Move;

mod models;
mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/root_moves")]
fn root_moves() -> Json<Vec<Move>> {
    use schema::moves::dsl::*;

    let connection = establish_connection();
    let results = moves.filter(parent.is_null())
        .load::<Move>(&connection)
        .expect("Error loading posts");
    Json(results)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/api", routes![root_moves])
    .launch();
}