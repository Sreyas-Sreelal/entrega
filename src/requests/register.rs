use crate::models::Users;
use crate::{register_user, DB};
use rocket::request::Form;
use rocket_contrib::json::{Json,JsonValue};

#[post("/user/register", data = "<formdata>")]
pub fn user_register(conn: DB, formdata: Form<Users>) -> Result<Json<JsonValue>, Json<JsonValue>> {
    register_user(&conn, formdata.into_inner())?;
    Ok(Json(json!({
        "Ok": true
    })))
}
