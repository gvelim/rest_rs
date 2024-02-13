use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Data, options, Orbit, Request, Response, Rocket};
use rocket::http::Header;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS header",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin","*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods","GET, POST, PUT, PATCH, DELETE, OPTIONS")
        );
        res.set_header(Header::new("Access-Control-Allow-Headers","*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials","true"));
    }
}

#[options("/<_..>")]
pub fn options() -> &'static str {
    ""
}