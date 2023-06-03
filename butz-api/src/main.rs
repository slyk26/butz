mod cors;
mod routes;
mod db;

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use cors::*;
use crate::db::DB;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let db = DB::new().await.unwrap();

    rocket::build()
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