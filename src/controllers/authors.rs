use rocket::{delete, get, post, put, State};
use rocket::http::Status;
use rocket::serde::{Deserialize,Serialize, json::Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::auth::AuthenticatedUser;
use crate::controllers::{Response, SuccessResponse, ErrorResponse};
use crate::entities::author;
use crate::entities::author::Model;

#[derive(Serialize,Deserialize)]
struct ResAuthor {
    id: i32,
    user_id: i32,
    first_name: String,
    last_name: String,
    bio: String,
}

#[derive(Serialize,Deserialize)]
struct ResAuthorsList {
    count: usize,
    list: Vec<ResAuthor>
}

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>, _user: AuthenticatedUser) -> Response<Json<ResAuthorsList>> {

    let list = author::Entity::find()
        .all(db.inner()).await?
        .iter()
        .map(|a| ResAuthor {
            id: a.id,
            user_id: a.user_id,
            first_name: a.first_name.to_owned(),
            last_name: a.last_name.to_owned(),
            bio: a.bio.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse(
        (Status::Ok, Json(ResAuthorsList { count: list.len(), list}))
    ))
}

#[derive(Deserialize)]
struct ReqAuthor {
    firstname: String,
    lastname: String,
    bio: String,
}
#[post("/", data="<req_author>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_author: Json<ReqAuthor>
) -> Response<Json<ResAuthor>>
{
    let author = author::ActiveModel {
        user_id: Set(user.id),
        first_name: Set(req_author.firstname.to_owned()),
        last_name: Set(req_author.lastname.to_owned()),
        bio: Set(req_author.bio.to_owned()),
        ..Default::default()
    }
        .insert(db.inner())
        .await?;

    Ok(SuccessResponse(
        (Status::Created, Json(ResAuthor {
            id: author.id,
            user_id: author.user_id,
            first_name: author.first_name.to_owned(),
            last_name: author.last_name.to_owned(),
            bio: author.bio.to_owned()
        }))
    ))
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    id: i32
) -> Response<Json<ResAuthor>> {
    let author = match author::Entity::find_by_id(id).one(db.inner()).await? {
        None => return Err(ErrorResponse(
            (Status::NotFound, format!("Author with id[{id}] was not found"))
        )),
        Some(a) => a
    };
    Ok(SuccessResponse(
        (Status::Found, Json(ResAuthor {
            id: author.id,
            user_id: author.user_id,
            first_name: author.first_name.to_owned(),
            last_name: author.last_name.to_owned(),
            bio: author.bio.to_owned()
        }))
    ))
}

#[put("/<id>")]
pub async fn update(id: u32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> Response<String> {
    todo!()
}