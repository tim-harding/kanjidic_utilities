#[macro_use]
extern crate rocket;

mod character_response;
mod field;
mod cache;
mod startup;
mod radicals_route;
mod kanji_route;

use rocket::{fairing::AdHoc};
use startup::{init_cache, init_db};
use kanji_route::kanji;
use radicals_route::radicals;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Connect Database", init_db))
        .attach(AdHoc::try_on_ignite("Create cache", init_cache))
        .mount("/", routes![kanji, radicals])
}