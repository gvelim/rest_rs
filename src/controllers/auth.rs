use std::time::SystemTime;
use bcrypt::{DEFAULT_COST, verify};
use jsonwebtoken::{EncodingKey, Header};
use rocket::{get, post, Responder, State};
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select};
use sea_orm::ActiveValue::Set;
use serde::Serialize;
use crate::AppConfig;
use crate::auth::AuthenticatedUser;
use crate::entities::{user, prelude::User};
use crate::entities::user::{Entity, Model};
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

#[derive(Deserialize,Serialize)]
pub struct Claims {
    pub(crate) sub: i32,
    role: String,
    exp: u64
}

#[post("/sign-in", data="<req_sign_in>")]
pub async fn sing_in(
    db: &State<DatabaseConnection>,
    cfg: &State<AppConfig>,
    req_sign_in: Json<ReqSignIn>
) -> Response<Json<ResSignIn>> {

    let usr = match User::find()
        .filter(user::Column::Email.eq(&req_sign_in.email))
        .one(db.inner()).await? {
        None => return Err(ErrorResponse(
            (Status::Unauthorized, "Invalid credentials".to_string())
        )),
        Some(usr) => usr
    };

    if !verify(&req_sign_in.password, &usr.password).unwrap() {
        return Err(ErrorResponse(
            (Status::Unauthorized, "Invalid credentials".to_string())
        ))
    }

    let claims = Claims {
        sub: usr.id,
        role: "user".to_string(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() + 4 * 3600
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.inner().jwt_secret.as_bytes())
    ).unwrap();

    Ok(SuccessResponse(
        (Status::Ok, Json(ResSignIn { token }))
    ))
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
        password: Set(bcrypt::hash(req_sign_up.password.to_owned(),DEFAULT_COST).unwrap()),
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

#[derive(Serialize,Deserialize)]
struct ResMe {
    id: u32,
    email: String,
    firstname: Option<String>,
    lastname: Option<String>
}

#[get("/me")]
pub async fn me(db: &State<DatabaseConnection>, user: AuthenticatedUser) -> Response<Json<ResMe>> {
    let u = User::find_by_id(user.id)
        .one(db.inner())
        .await?
        .unwrap();
    let me = ResMe {
        id: user.id as u32,
        email: u.email,
        firstname: u.first_name,
        lastname: u.last_name,
    };
    Ok(SuccessResponse((Status::Ok, Json(me))))
}