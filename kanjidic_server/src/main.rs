#[macro_use]
extern crate rocket;

use rocket_sync_db_pools::{database, diesel};

#[database("postgres_wsl2")]
struct DbConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build().attach(DbConn::fairing())
}
