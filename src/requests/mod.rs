pub mod get_product_details;
pub mod get_random_product;
pub mod login;
pub mod product_add;
pub mod register;
pub mod search_product;
pub mod product_remove;

use serde_derive::Deserialize;

//LoginForm
//used as structure for serialising login request forms
#[derive(FromForm, Deserialize, Debug)]
pub struct LoginForm {
    pub name: String,
    pub password: String,
}

//ProductSearchPayload
//Format of request data for searching a product 
#[derive(Deserialize)]
pub struct ProductSearchPayload {
    pub name: Option<String>,
    pub min_rate: Option<f32>,
    pub max_rate: Option<f32>,
    pub min_price: Option<f32>,
    pub max_price: Option<f32>,
}

//AuthRequestPayload
//A general type structure for requests that requires authentication
#[derive(Deserialize)]
pub struct AuthRequestPayload<T>{
    pub token: String,
    pub data:T
}


