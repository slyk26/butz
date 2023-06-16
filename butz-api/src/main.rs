mod cors;
mod db;
mod utils;
mod models;
mod routes;

#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;
use rocket::{Build, Config, Rocket};
use rocket::http::Status;
use cors::*;
use crate::db::DB;
use crate::routes::users;

#[get("/")]
fn healthcheck() -> Status {
    Status::Ok
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let db = DB::new().await.unwrap();
    let env = std::env::var("ENV").unwrap_or(String::from("DEV"));
    let mut config = Config { ..Config::debug_default() };

    if env.eq("PROD") {
         config = Config {
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            ..Config::release_default()
        };
    }

    // TODO: add limit for users
    rocket::custom(config)
        .manage(db)
        .attach(Cors)
        .mount("/", routes![healthcheck])
        .mount("/api", users::routes())
}