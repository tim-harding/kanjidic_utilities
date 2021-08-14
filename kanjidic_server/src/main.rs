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
use rocket::fairing::AdHoc;
use startup::{init_cache, init_db};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Connect Database", init_db))
        .attach(AdHoc::try_on_ignite("Create cache", init_cache))
        .mount(
            "/",
            routes![kanji, decomposition, radicals_all, radicals_some],
        )
}
