use crate::database::core::fetch_random_product;
use crate::database::core::DB;
use rocket_contrib::json::{Json, JsonValue};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Limit {
    pub limit: i64,
}

#[post("/product/get_random_product", data = "<limit>")]
pub fn get_random_product(conn: DB, limit: Json<Limit>) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let products = fetch_random_product(&conn, limit.into_inner().limit)?;
    Ok(Json(json!({
        "Ok": true,
        "products":products
    })))
}
