use crate::auth::auth_user;
use crate::database::core::DB;
use crate::requests::LoginForm;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::{Json, JsonValue};

//TODO: pass admin status too 
#[post("/user/auth", data = "<data>")]
pub fn user_login(
    mut cookies: Cookies,
    conn: DB,
    data: Json<LoginForm>,
) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let token = auth_user(&conn, data.into_inner())?;
    cookies.add(Cookie::new("jwt", token.clone()));
    Ok(Json(json!({
        "Ok": true,
        "token": token
    })))
}
