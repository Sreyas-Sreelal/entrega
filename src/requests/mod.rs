pub mod login;
pub mod register;
use serde_derive::Deserialize;


//LoginForm 
//used as structure for serialising login request forms
#[derive(FromForm,Deserialize,Debug)]
pub struct LoginForm {
    pub name: String,
    pub password: String
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