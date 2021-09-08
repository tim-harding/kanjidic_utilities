#[macro_use]
extern crate rocket;

mod cache;
mod character_response;
mod cors;
mod decomposition_route;
mod field;
mod kanji_route;
mod radical_response;
mod radicals_route;
mod shared;
mod startup;
mod translation_route;

use decomposition_route::decomposition;
use kanji_route::kanji;
use radicals_route::{radicals_all, radicals_some};
use rocket::fairing::AdHoc;
use startup::{init_cache, init_db};
use translation_route::translation;

use crate::cors::Cors;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Connect Database", init_db))
        .attach(AdHoc::try_on_ignite("Create cache", init_cache))
        .attach(Cors)
        .mount(
            "/",
            routes![
                kanji,
                decomposition,
                radicals_all,
                radicals_some,
                translation
            ],
        )
}
