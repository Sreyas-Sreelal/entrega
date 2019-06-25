use crate::database::schema::{admin, order, product, user};
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, FromForm, Insertable, Debug, Deserialize)]
#[primary_key(user_id)]
#[table_name = "user"]
pub struct User {
    pub user_id: Option<i32>,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub display_name: String,
    pub address: String,
}

#[derive(Identifiable, Queryable, FromForm, Associations, Insertable)]
#[belongs_to(User, foreign_key = "user_id")]
#[primary_key(user_id)]
#[table_name = "admin"]
pub struct Admin {
    user_id: Option<i32>,
}

#[derive(Identifiable,Queryable, FromForm, Insertable, Deserialize, Serialize, QueryableByName)]
#[primary_key(product_id)]
#[table_name = "product"]
pub struct Product {
    pub product_id: Option<String>,
    product_name: String,
    pub description: Option<String>,
    price: f32,
    rating: Option<f32>,
}

#[derive(Queryable, FromForm, Associations, Insertable)]
#[table_name = "order"]
pub struct Order {
    order_id: i32,
    product_id: Option<String>,
    user_id: i32,
    ordered_date: i32,
    expected_date: i32,
}
