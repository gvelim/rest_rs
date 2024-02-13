pub mod auth;
pub mod authors;
pub mod books;

use std::str::FromStr;
use rocket::http::Status;
use rocket::Responder;
use sea_orm::DbErr;

#[derive(Responder)]
pub struct SuccessResponse<T>(pub(Status,T));

#[derive(Responder)]
pub struct ErrorResponse(pub(Status,String));

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;

impl From<DbErr> for ErrorResponse {
    fn from(value: DbErr) -> Self {
        ErrorResponse((Status::InternalServerError, value.to_string()))
    }
}