use rocket::{post, Responder, State};
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use super::Response;

#[derive(Deserialize)]
pub struct ReqSignIn {
    email: String,
    password: String
}

#[derive(Serialize, Deserialize, Responder)]
pub struct ResSignIn {
    token: String
}

#[post("/sing-in", data="<req_sign_in>")]
pub async fn sing_in(db: &State<DatabaseConnection>, req_sign_in: Json<ReqSignIn>) -> Response<ResSignIn> {

    todo!()
}

#[derive(Deserialize)]
pub struct ReqSignUp {
    email: String,
    password: String,
    firstname: Option<String>,
    lastname: Option<String>
}

#[post("/sing-up", data = "<req_sign_up>")]
pub async fn sing_up(db: &State<DatabaseConnection>, req_sign_up: Json<ReqSignUp>) -> Response<String> {

    todo!()
}