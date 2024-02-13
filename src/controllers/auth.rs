use rocket::post;
use super::Response;

#[post("/sing-in")]
pub async fn sing_in() -> Response<String> {
    todo!()
}

#[post("/sing-up")]
pub async fn sing_up() -> Response<String> {
    todo!()
}