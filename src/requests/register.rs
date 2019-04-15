use crate::database::core::{register_user, DB};
use crate::database::models::Users;
use rocket_contrib::json::{Json, JsonValue};

#[post("/user/register", data = "<formdata>")]
pub fn user_register(conn: DB, formdata: Json<Users>) -> Result<Json<JsonValue>, Json<JsonValue>> {
    register_user(&conn, formdata.into_inner())?;
    Ok(Json(json!({
        "Ok": true
    })))
}
