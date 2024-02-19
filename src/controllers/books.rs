use rocket::{delete, get, post, put, State};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeUtc;
use crate::auth::AuthenticatedUser;
use crate::controllers::{ErrorResponse, Response, SuccessResponse};
use crate::entities::{author, book};
use crate::entities::book::Model;
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

#[derive(Deserialize)]
struct ReqBook {
    author_id: i32,
    title: String,
    cover: String,
    year: String,
}

#[post("/", data="<req_book>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_book: Json<ReqBook>
) -> Response<Json<ResBook>> {
    let book = book::ActiveModel {
        user_id: Set(user.id),
        author_id: Set(req_book.author_id),
        title: Set(req_book.title.to_owned()),
        cover: Set(req_book.cover.to_owned()),
        year: Set(req_book.year.to_owned()),
        ..Default::default()
    }
        .insert(db.inner())
        .await?;

    Ok(SuccessResponse(
        (Status::Created, Json(ResBook{
            id: book.id,
            author_id: book.author_id,
            cover: book.cover.to_owned(),
            title: book.title.to_owned(),
            year: book.year
        }))
    ))
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    id: i32
) -> Response<Json<ResBook>> {
    let book = match book::Entity::find_by_id(id).one(db.inner()).await? {
        None => return Err(ErrorResponse(
            (Status::NotFound, format!("Book with id[{id}] was not found"))
        )),
        Some(a) => a
    };
    Ok(SuccessResponse(
        (Status::Found, Json(ResBook{
            id: book.id,
            author_id: book.author_id,
            cover: book.cover.to_owned(),
            title: book.title.to_owned(),
            year: book.year
        }))
    ))
}

#[put("/<id>", data="<req_book>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
    req_book: Json<ReqBook>
) -> Response<Json<ResBook>> {
    let mut book : book::ActiveModel = match Book::find_by_id(id).one(db.inner()).await? {
        None => return Err(ErrorResponse(
            (Status::NotFound, format!("Book with id[{id}] was not found"))
        )),
        Some(b) => b.into()
    };
    book.title = Set(req_book.title.to_owned());
    book.cover = Set(req_book.cover.to_owned());
    book.year = Set(req_book.year.to_owned());
    book.updated_at = Set(Some(DateTimeUtc::from(std::time::SystemTime::now()).to_string()));

    let book = book.update(db.inner()).await?;

    Ok(SuccessResponse(
        (Status::Ok, Json(ResBook{
            id: book.id,
            author_id: book.author_id,
            cover: book.cover.to_owned(),
            title: book.title.to_owned(),
            year: book.year
        }))
    ))
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> Response<String> {
    todo!()
}