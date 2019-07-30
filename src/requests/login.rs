use crate::auth::auth_user;
use crate::database::core::DB;
use crate::requests::LoginForm;
use rocket_contrib::json::{Json, JsonValue};

//TODO: pass admin status too
#[post("/user/auth", data = "<data>")]
pub fn user_login(conn: DB, data: Json<LoginForm>) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let (token, user, admin) = auth_user(&conn, data.into_inner())?;

    Ok(Json(json!({
        "Ok": true,
        "token": token,
        "user": user,
        "admin":admin,
    })))
}
