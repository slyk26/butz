mod cors;
mod routes;
mod db;
mod utils;
mod models;

#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;
use rocket::{Build, Config, Rocket};
use cors::*;
use crate::db::DB;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
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
        .mount("/", routes![hello])
        .mount("/api", routes![
            routes::users::get_all,
            routes::users::get,
            routes::users::add,
            routes::users::delete,
            routes::users::update
        ])
}