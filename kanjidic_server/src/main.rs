#[macro_use]
extern crate rocket;

use mongodb::{options::ClientOptions, Client};
use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};

#[launch]
fn rocket() -> _ {
    rocket::build().attach(AdHoc::try_on_ignite("Connect Database", init_db))
}

async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    let db_url = match std::env::var("mongodb_url") {
        Ok(url) => url,
        Err(err) => {
            error!("Failed to get `mongodb_url` environment variable: {}", err);
            return Err(rocket);
        }
    };
    let client_options = match ClientOptions::parse(db_url).await {
        Ok(options) => options,
        Err(err) => {
            error!("Failed to parse mongodb client options: {}", err);
            return Err(rocket);
        }
    };
    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to get mongodb client: {}", err);
            return Err(rocket);
        }
    };
    Ok(rocket.manage(client))
}
