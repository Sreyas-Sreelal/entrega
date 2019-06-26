use crate::database::core::fetch_product;
use crate::database::core::DB;
use rocket_contrib::json::{Json, JsonValue};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct GetProductPayload {
    pub id: Option<String>,
}

#[post("/product/get_product_details", data = "<payload>")]
pub fn get_product_details(
    conn: DB,
    payload: Json<GetProductPayload>,
) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let product = fetch_product(&conn, payload.into_inner().id)?;
    Ok(Json(json!({
        "Ok": true,
        "product":product
    })))
}
