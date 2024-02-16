use rocket::{delete, get, post, put, State};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use crate::auth::AuthenticatedUser;
use crate::controllers::{Response, SuccessResponse};
use crate::entities::book;
use crate::entities::prelude::Book;


#[derive(Serialize,Deserialize)]
pub struct ResBook {
    id: i32,
    author_id: i32,
    title: String,
    cover: String,
    year: String,
}

#[derive(Serialize,Deserialize)]
pub struct ResBookList {
    total: usize,
    list: Vec<ResBook>
}

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>, _user: AuthenticatedUser) -> Response<Json<ResBookList>> {

    let bl = Book::find()
        .order_by_asc(book::Column::CreatedAt)
        .all(db.inner()).await?
        .iter()
        .map(|b| ResBook{
            id: b.id,
            author_id: b.author_id,
            title: b.title.to_owned(),
            cover: b.cover.to_owned(),
            year: b.year.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse(
        (Status::Ok, Json(ResBookList {
            total: bl.len(),
            list: bl
        }))
    ))
}

#[post("/")]
pub async fn create() -> Response<String> {
    todo!()
}

#[get("/<id>")]
pub async fn show(id: u32) -> Response<String> {
    todo!()
}

#[put("/<id>")]
pub async fn update(id: u32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> Response<String> {
    todo!()
}