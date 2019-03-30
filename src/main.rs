#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

pub mod database;
pub mod requests;

use crate::database::core::DB;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![requests::register::user_register])
        .attach(DB::fairing())
        .launch();
}
