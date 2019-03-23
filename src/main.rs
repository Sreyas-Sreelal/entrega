#![feature(proc_macro_hygiene, decl_macro)]

pub mod models;
pub mod requests;
pub mod schema;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use crate::models::Users;

#[database("entrega")]
pub struct DB(SqliteConnection);

pub fn db_connect() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn register_user(conn: &DB, doc: Users) -> Result<(), diesel::result::Error> {
    use schema::users;

    diesel::insert_into(users::table)
        .values(&doc)
        .execute(conn as &SqliteConnection)?;

    Ok(())
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![requests::register::user_register])
        .attach(DB::fairing())
        .launch();
}
