#![feature(plugin, custom_derive)]
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
mod rest_api;
mod schema;
mod utils;
mod web_interface;

use rocket_contrib::Template;

fn main() {
    //env_logger::init();

    rocket::ignite()
        .mount(
            "/",
            routes![
                web_interface::index,
                web_interface::get_move,
                web_interface::delete_move,
                web_interface::create_move,
                web_interface::create_root_move,
                web_interface::create_move_from_form
            ],
        )
        .mount("/static", routes![web_interface::files])
        .mount(
            "/api/moves",
            routes![
                rest_api::create_move,
                rest_api::request_move,
                rest_api::update_move,
                rest_api::delete_move,
                rest_api::root_moves
            ],
        )
        .attach(Template::fairing())
        .launch();
}
