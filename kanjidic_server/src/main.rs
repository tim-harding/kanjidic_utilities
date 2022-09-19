#[macro_use]
extern crate rocket;

mod cache;
mod character_response;
mod cors;
mod decomposition_route;
mod field;
mod kanji_literals_route;
mod radical_response;
mod radicals_route;
mod shared;
mod startup;
mod translation_route;

use crate::cors::Cors;
use clap::Parser;
use decomposition_route::decomposition;
use kanji_literals_route::kanji;
use radicals_route::{radicals_all, radicals_some};
use rocket::fairing::AdHoc;
use startup::init_cache;
use translation_route::translation;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    kanji_path: String,
}

#[launch]
fn rocket() -> _ {
    let cli = Cli::parse();
    rocket::build()
        .attach(AdHoc::try_on_ignite("Create cache", |rocket| {
            init_cache(rocket, cli.kanji_path)
        }))
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
