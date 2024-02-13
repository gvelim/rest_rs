use rocket::{delete, get, post, put};
use crate::controllers::Response;


#[get("/")]
pub async fn index() -> Response<String> {
    todo!()
}

#[post("/")]
pub async fn create() -> Response<String> {
    todo!()
}

#[get("/id")]
pub async fn show() -> Response<String> {
    todo!()
}

#[put("/id")]
pub async fn update() -> Response<String> {
    todo!()
}

#[delete("/id")]
pub async fn delete() -> Response<String> {
    todo!()
}