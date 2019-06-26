pub mod get_product_details;
pub mod get_random_product;
pub mod login;
pub mod product_add;
pub mod register;
pub mod search_product;

use serde_derive::Deserialize;

//LoginForm
//used as structure for serialising login request forms
#[derive(FromForm, Deserialize, Debug)]
pub struct LoginForm {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ProductSearchPayload {
    pub name: Option<String>,
    pub min_rate: Option<f32>,
    pub max_rate: Option<f32>,
    pub min_price: Option<f32>,
    pub max_price: Option<f32>,
}

/*
//RegisterForm
//used as structure for serialising register request forms
#[derive(FromForm,Debug,Insertable)]
pub struct RegisterForm {
    user_name: String,
    email: String,
    display_name: String,
    password: String,
    address: String
}*/
