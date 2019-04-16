use crate::database::core::{register_user, DB};
use crate::database::models::User;
use rocket_contrib::json::{Json, JsonValue};

#[post("/user/register", data = "<formdata>")]
pub fn user_register(conn: DB, formdata: Json<User>) -> Result<Json<JsonValue>, Json<JsonValue>> {
    register_user(&conn, formdata.into_inner())?;
    Ok(Json(json!({
        "Ok": true
    })))
}
