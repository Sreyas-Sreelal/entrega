use crate::database::core::DB;
use crate::auth::auth_user;
use rocket_contrib::json::{Json, JsonValue};
use crate::requests::LoginForm;

#[post("/user/auth", data = "<data>")]
pub fn user_login(conn: DB, data: Json<LoginForm>) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let token = auth_user(&conn, data.into_inner())?;
    Ok(Json(json!({
        "Ok": true,
        "token": token
    })))    
}
