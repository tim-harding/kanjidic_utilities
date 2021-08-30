#[macro_use]
extern crate rocket;

mod cache;
mod character_response;
mod decomposition_route;
mod field;
mod kanji_route;
mod radicals_route;
mod startup;

use decomposition_route::decomposition;
use kanji_route::kanji;
use radicals_route::{radicals_all, radicals_some};
use rocket::{Request, Response, fairing::{AdHoc, Fairing, Info, Kind}, http::Header};
use startup::{init_cache, init_db};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Connect Database", init_db))
        .attach(AdHoc::try_on_ignite("Create cache", init_cache))
        .attach(Cors {})
        .mount(
            "/",
            routes![kanji, decomposition, radicals_all, radicals_some],
        )
}

struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "false"));
    }
}