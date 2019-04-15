static KEY: &'static [u8; 16] = include_bytes!("../../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

use jsonwebtoken::{encode,Header};
use rocket_contrib::json::{Json, JsonValue};
use crate::database::core::DB;
use crate::database::models::Users;
use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::requests::LoginForm;
#[derive(Debug, RustcEncodable, RustcDecodable)]
struct AuthData {
    iat: i64,
    exp: i64,
    user: String,
    admin: bool,
}

/*
impl AuthData {
    fn is_admin(&self) -> bool {
        self.admin
    }

    fn is_expired(&self) -> bool {
        let now = time::get_time().sec;
        now >= self.exp
    }
}
*/

fn jwt_generate(user: String, admin:bool) -> String {
    let now = time::get_time().sec;
    let payload = AuthData {
        iat: now,
        exp: now + ONE_WEEK,
        user: user,
        admin: admin,
    };

    encode(Header::default(), &payload, KEY).unwrap()
}

pub fn auth_user(conn: &DB, data: LoginForm) -> Result<String, Json<JsonValue>> {
    let username = data.name;
    use crate::database::schema::users::dsl::*;
    let user = match users.filter(user_name.eq(&username)).first::<Users>(conn as &SqliteConnection) {
        Ok(u) => { 
           u
        }
        Err(err) => return Err(Json(json!({
            "Ok":false,
            "message":err.to_string()
        }))),
    };

    println!("[Debug] user authention form {:?}",user);

    if user.password != data.password {
        return Err(Json(json!({
            "Ok":false,
            "message":"Authentication Failed"
        })));
    }
    let token = jwt_generate(username,false);
    return Ok(token);
}