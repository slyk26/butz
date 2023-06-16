use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use crate::db::DB;
use crate::models::User;

const TABLE: &str = "users";

#[get("/users")]
async fn get_all(db: &State<DB>) -> Result<Json<Vec<User>>, Status> {
    super::get_all(db, TABLE).await
}

#[get("/users/<key>")]
async fn get(db: &State<DB>, key: &str) -> Result<Json<User>, Status> {
    super::get(db, key).await
}

#[post("/users", data = "<body>")]
async fn post(db: &State<DB>, body: User) -> Result<(Status, Json<User>), Status> {
    super::post(db, body, TABLE).await
}

#[delete("/users/<key>")]
async fn delete(db: &State<DB>, key: &str) -> Result<(Status, Json<User>), Status> {
    super::delete(db, key).await
}

#[put("/users/<key>", data= "<body>")]
async fn put(db: &State<DB>, key: &str, body: User) -> Result<Status, Status> {
    super::put(db, key, body).await
}

pub fn routes() -> Vec<Route> {
    routes![get_all, get, post, put, delete]
}