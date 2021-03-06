#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

pub mod auth;
pub mod database;
pub mod requests;

use crate::auth::cors::CORS;
use crate::database::core::DB;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount(
            "/",
            routes![
                requests::register::user_register,
                requests::login::user_login,
                requests::product_add::product_add,
                requests::get_random_product::get_random_product,
                requests::search_product::search_product,
                requests::get_product_details::get_product_details,
                requests::product_remove::product_remove,
            ],
        )
        .attach(DB::fairing())
        .attach(CORS())
        .launch();
}
