use crate::auth::token_validate;
use crate::database::core::{create_product, DB};
use crate::database::models::Product;
use rocket::http::Cookies;
use rocket_contrib::json::{Json, JsonValue};

#[post("/product/new", data = "<formdata>")]
pub fn product_add(
    cookies: Cookies,
    conn: DB,
    formdata: Json<Product>,
) -> Result<Json<JsonValue>, Json<JsonValue>> {
    token_validate(true, cookies)?;
    let product = create_product(&conn, formdata.into_inner())?;
    Ok(Json(json!({
        "Ok": true,
        "product_id": product
    })))
}
