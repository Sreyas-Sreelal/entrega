use crate::database::schema::{orders, products, users,admins};
use serde_derive::Deserialize;

#[derive(Queryable, FromForm, Insertable,Debug,Deserialize)]
#[table_name = "users"]
pub struct Users {
    user_id: Option<i32>,
    user_name: String,
    pub password: String,
    email: String,
    display_name: String,
    address: String,
}

#[derive(Queryable, FromForm, Insertable)]
#[table_name = "admins"]
pub struct Admins{
    user_id: i32,
}

#[derive(Queryable, FromForm, Insertable)]
#[table_name = "products"]
pub struct Products {
    product_id: i32,
    product_name: String,
    price: f32,
    rating: f32,
}

#[derive(Queryable, FromForm, Insertable)]
#[table_name = "orders"]
pub struct Orders {
    order_id: i32,
    product_id: i32,
    user_id: i32,
    ordered_date: i32,
    expected_date: i32,
}
