mod migrator;
mod db;
mod entities;
mod controllers;
mod fairings;
mod auth;

use rocket::{routes, launch, get, Responder, State};
use rocket::http::Status;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use crate::controllers::SuccessResponse;
use crate::migrator::Migrator;

pub struct AppConfig {
    db_database: String,
    jwt_secret: String
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").unwrap_or("db".to_string()),
            jwt_secret: std::env::var("BOOKSTORE_JWT_SECRET")
                .expect("Cannot find env variable BOOKSTORE_JWT_SECRET")
        }
    }
}

#[get("/")]
fn index() -> controllers::Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello World!".to_string())))
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let cfg = AppConfig::default();
    let db = db::connect(&cfg).await.unwrap();
    
    Migrator::up(&db, None).await.unwrap();

    rocket::build()
        .attach(fairings::cors::CORS)
        .manage(db)
        .manage(cfg)
        .mount("/", routes![fairings::cors::options])
        .mount("/", routes![index])
        .mount("/auth", routes![
            controllers::auth::sing_in,
            controllers::auth::sing_up,
            controllers::auth::me
        ])
        .mount("/authors", routes![
            controllers::authors::index,
            controllers::authors::create,
            controllers::authors::show,
            controllers::authors::update,
            controllers::authors::delete
        ])
        .mount("/books", routes![
            controllers::books::index,
            controllers::books::create,
            controllers::books::show,
            controllers::books::update,
            controllers::books::delete
        ])
}
