use crate::auth::token_validate;
use crate::database::core::{delete_product, DB};
use rocket_contrib::json::{Json, JsonValue};
use crate::requests::AuthRequestPayload;

//Removes product from the database
//needs product id
#[post("/product/remove", data = "<formdata>")]
pub fn product_remove(
    conn: DB,
    formdata: Json<AuthRequestPayload<String>>,
) -> Result<Json<JsonValue>, Json<JsonValue>> {
	
	let payload = formdata.into_inner();
    token_validate(true, &payload.token)?;
    
    delete_product(&conn, payload.data)?;
    
    Ok(Json(json!({
        "Ok": true
    })))
}
