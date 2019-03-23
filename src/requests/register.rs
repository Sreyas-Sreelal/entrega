use crate::models::Users;
use crate::{register_user, DB};
use rocket::request::Form;
use rocket::response::Redirect;

//TODO:return JSON data as repsonse instead of redirect
#[post("/user/register", data = "<formdata>")]
pub fn user_register(conn: DB, formdata: Form<Users>) -> Result<Redirect, diesel::result::Error> {
    register_user(&conn, formdata.into_inner())?;
    Ok(Redirect::to("/"))
}
