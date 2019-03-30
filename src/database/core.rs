use crate::database::models::Users;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rocket_contrib::json::{Json, JsonValue};
use std::env;

#[database("entrega")]
pub struct DB(SqliteConnection);

pub fn db_connect() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn register_user(conn: &DB, doc: Users) -> Result<(), Json<JsonValue>> {
    use crate::database::schema::users;

    match diesel::insert_into(users::table)
        .values(&doc)
        .execute(conn as &SqliteConnection)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    }
}
