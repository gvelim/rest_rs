pub mod auth;
pub mod authors;
pub mod books;

use rocket::http::Status;
use rocket::Responder;

#[derive(Responder)]
pub struct SuccessResponse<T>(pub(Status,T));

#[derive(Responder)]
pub struct ErrorResponse<T>(pub(Status,T));

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse<T>>;

