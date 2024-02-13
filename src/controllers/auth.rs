use rocket::{post, Responder, State};
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde::Serialize;
use crate::entities::{user, prelude::User};
use super::{ErrorResponse, Response, SuccessResponse};

#[derive(Deserialize)]
pub struct ReqSignIn {
    email: String,
    password: String
}

#[derive(Serialize, Deserialize, Responder)]
pub struct ResSignIn {
    token: String
}

#[post("/sign-in", data="<req_sign_in>")]
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

#[post("/sign-up", data = "<req_sign_up>")]
pub async fn sing_up(db: &State<DatabaseConnection>, req_sign_up: Json<ReqSignUp>) -> Response<String> {

    if User::find()
        .filter(user::Column::Email.eq(&req_sign_up.email))
        .one(db.inner()).await?
        .is_some() {
        return Err(ErrorResponse(
            (Status::UnprocessableEntity, "An account already exists for this email address".to_string())
        ))
    }

    User::insert(user::ActiveModel {
        email: Set(req_sign_up.email.to_owned()),
        password: Set(req_sign_up.password.to_owned()),
        first_name: Set(req_sign_up.firstname.to_owned()),
        last_name: Set(req_sign_up.lastname.to_owned()),
        ..Default::default()
    })
        .exec(db.inner())
        .await?;

    Ok(SuccessResponse(
        (Status::Created, "Account Created!".to_string())
    ))
}