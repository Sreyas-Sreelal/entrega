use crate::auth::token_decode;
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
    let token = cookies.get("jwt");
    if token == None {
        return Err(Json(json!({
            "Ok":false,
            "message":"Empty token"
        })));
    }

    let auth = token_decode(token.unwrap().value())?;
    if !auth.is_admin() {
        return Err(Json(json!({
            "Ok": false,
            "message": "Authentication needed"
        })));
    }

    if auth.is_expired() {
        return Err(Json(json!({
            "Ok": false,
            "message": "Token Expired"
        })));
    }

    let product = create_product(&conn, formdata.into_inner())?;
    Ok(Json(json!({
        "Ok": true,
        "product_id": product
    })))
}
