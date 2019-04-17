use crate::database::models::{Admin, Product, User};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rocket_contrib::json::{Json, JsonValue};
use std::env;
use uuid::Uuid;

#[database("entrega")]
pub struct DB(SqliteConnection);

pub fn db_connect() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn check_admin(conn: &DB, user_fetched: &User) -> bool {
    match Admin::belonging_to(user_fetched).first::<Admin>(conn as &SqliteConnection) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn fetch_user(conn: &DB, username: &str) -> Result<User, Json<JsonValue>> {
    use crate::database::schema::user::dsl::*;
    match user
        .filter(user_name.eq(&username))
        .first::<User>(conn as &SqliteConnection)
    {
        Ok(u) => Ok(u),
        Err(err) => Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    }
}

pub fn fetch_product(conn: &DB,limit:i64) -> Result<Vec<Product>, Json<JsonValue>> {
    use crate::database::schema::product::dsl::*;

    no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

    match product
        .order(RANDOM)
        .limit(limit)
        .load::<Product>(conn as & SqliteConnection) {
            Ok(u) => Ok(u),
            Err(err) => Err(Json(json!({
                "Ok":false,
                "message":err.to_string()
            }))),
    
    }
}

pub fn register_user(conn: &DB, doc: User) -> Result<(), Json<JsonValue>> {
    use crate::database::schema::user;

    match diesel::insert_into(user::table)
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

pub fn create_product(conn: &DB, mut item: Product) -> Result<String, Json<JsonValue>> {
    use crate::database::schema::product;
    let id = Uuid::new_v4().to_hyphenated().to_string();
    item.product_id = Some(id);
    match diesel::insert_into(product::table)
        .values(&item)
        .execute(conn as &SqliteConnection)
    {
        Ok(_) => Ok(String::from(item.product_id.unwrap())),
        Err(err) => Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    }
}
