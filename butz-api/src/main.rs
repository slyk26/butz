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

    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::release_default()
    };

    rocket::custom(config)
        .manage(db)
        .attach(Cors)
        .mount("/", routes![
            hello,
            routes::users::get_all,
            routes::users::get,
            routes::users::add,
            routes::users::delete,
            routes::users::update])
}