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

mod models;
mod routes;
mod schema;
mod utils;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api", routes![root_moves, request_move])
        .launch();
}
