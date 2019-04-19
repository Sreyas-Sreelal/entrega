use crate::database::core::db_search_product;
use crate::database::core::DB;
use crate::requests::ProductSearchPayload;
use rocket_contrib::json::{Json, JsonValue};

#[post("/product/search", data = "<payload>")]
pub fn search_product(
    conn: DB,
    payload: Json<ProductSearchPayload>,
) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let products = db_search_product(&conn, payload.into_inner())?;
    Ok(Json(json!({
        "Ok": true,
        "products":products
    })))
}
