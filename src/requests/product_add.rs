use crate::auth::token_validate;
use crate::database::core::{create_product, DB};
use crate::database::models::Product;
use crate::requests::AuthRequestPayload;
use rocket_contrib::json::{Json, JsonValue};

#[post("/product/new", data = "<formdata>")]
pub fn product_add(
    conn: DB,
    formdata: Json<AuthRequestPayload<Product>>,
) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let payload = formdata.into_inner();
    token_validate(true, &payload.token)?;

    let product = create_product(&conn, payload.data)?;
    Ok(Json(json!({
        "Ok": true,
        "product_id": product
    })))
}
