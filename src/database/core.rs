use crate::database::models::{Admin, Product, User};
use crate::requests::ProductSearchPayload;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};
use std::env;
use uuid::Uuid;

#[database("entrega")]
pub struct DB(MysqlConnection);

pub fn db_connect() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn check_admin(conn: &DB, user_fetched: &User) -> bool {
    Admin::belonging_to(user_fetched)
        .first::<Admin>(conn as &MysqlConnection)
        .is_ok()
}

pub fn fetch_user(conn: &DB, username: &str) -> Result<User, Json<JsonValue>> {
    use crate::database::schema::user::dsl::*;
    match user
        .filter(user_name.eq(&username.to_ascii_lowercase()))
        .first::<User>(conn as &MysqlConnection)
    {
        Ok(u) => Ok(u),
        Err(err) => Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    }
}

pub fn register_user(conn: &DB, mut doc: User) -> Result<(), Json<JsonValue>> {
    use crate::database::schema::user;
    doc.user_name = doc.user_name.to_ascii_lowercase();

    match diesel::insert_into(user::table)
        .values(&doc)
        .execute(conn as &MysqlConnection)
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
        .execute(conn as &MysqlConnection)
    {
        Ok(_) => Ok(item.product_id.unwrap()),
        Err(err) => Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    }
}

pub fn fetch_random_product(conn: &DB, limit: i64) -> Result<Vec<Product>, Json<JsonValue>> {
    use crate::database::schema::product::dsl::*;

    no_arg_sql_function!(RAND, (), "Represents the MySQL RAND() function");
    match product
        .order(RAND)
        .limit(limit)
        .load::<Product>(conn as &MysqlConnection)
    {
        Ok(u) => Ok(u),
        Err(err) => Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    }
}

pub fn fetch_product(conn: &DB, id: Option<String>) -> Result<Product, Json<JsonValue>> {
    use crate::database::schema::product::dsl::*;
    match product
        .filter(product_id.eq(&id))
        .first::<Product>(conn as &MysqlConnection)
    {
        Ok(p) => Ok(p),
        Err(err) => Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    }
}

pub fn db_search_product(
    conn: &DB,
    payload: ProductSearchPayload,
) -> Result<Vec<Product>, Json<JsonValue>> {
    use crate::database::schema::product::dsl::*;
    if payload.name.is_some() {
        match product
            .filter(product_name.like(format!("%{}%", payload.name.unwrap())))
            .load::<Product>(conn as &MysqlConnection)
        {
            Ok(products) => return Ok(products),
            Err(err) => {
                return Err(Json(json!({
                    "Ok":false,
                    "message":err.to_string()
                })));
            }
        };
    }

    Err(Json(json!({
        "Ok":false,
        "message":"UnRecognised payload"
    })))
}
