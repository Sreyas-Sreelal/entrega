use crate::database::core::{check_admin, fetch_user, DB};
use crate::requests::LoginForm;
use jsonwebtoken::{decode, encode, Algorithm, Header};
use rocket_contrib::json::{Json, JsonValue};

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct AuthData {
    iat: i64,
    exp: i64,
    user: String,
    admin: bool,
}

static KEY: &'static [u8; 16] = include_bytes!("../../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

impl AuthData {
    pub fn is_admin(&self) -> bool {
        self.admin
    }

    pub fn is_expired(&self) -> bool {
        let now = time::get_time().sec;
        now >= self.exp
    }
}

fn token_generate(user: String, admin: bool) -> String {
    let now = time::get_time().sec;
    let payload = AuthData {
        iat: now,
        exp: now + ONE_WEEK,
        user: user,
        admin: admin,
    };

    encode(Header::default(), &payload, KEY).unwrap()
}

pub fn token_decode(token: &str) -> Result<AuthData, Json<JsonValue>> {
    println!("token {}", token);
    match decode::<AuthData>(&token, KEY, Algorithm::HS256) {
        Ok(c) => Ok(c.claims),
        Err(err) => {
            return Err(Json(json!({
                "Ok":false,
                "message":err.to_string()
            })));
        }
    }
}

pub fn auth_user(conn: &DB, data: LoginForm) -> Result<String, Json<JsonValue>> {
    let username = data.name;

    let user_fetched = fetch_user(conn, &username)?;
    println!("[Debug] user authention form {:?}", user_fetched);

    if user_fetched.password != data.password {
        return Err(Json(json!({
            "Ok":false,
            "message":"Authentication Failed"
        })));
    }

    let admin = check_admin(conn, &user_fetched);

    println!("admin {}", admin);

    let token = token_generate(username, admin);

    return Ok(token);
}
