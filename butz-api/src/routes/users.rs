use std::io::Error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::db::DB;
use crate::models::User;
use crate::utils::new_error;

const TABLE: &str = "users";

#[get("/users")]
pub async fn get_all(db: &State<DB>) -> Result<Json<Vec<User>>, Error> {
    Ok(Json(db.get_all(TABLE)
        .await.map_err(|_| new_error("Could not get users"))?))
}

#[get("/users/<key>")]
pub async fn get(db: &State<DB>, key: &str) -> Result<Json<User>, Error> {
    if let Ok(Some(user)) = db.get(key).await {
        return Ok(Json(user))
    }
    Err(new_error("Could not find user"))
}

#[post("/users", data = "<user>")]
pub async fn add(db: &State<DB>, user: User) -> Result<(Status, Json<User>), Error> {
    if let Ok(inserted) = db.add(TABLE, user).await {
        return Ok((Status::Created, Json(inserted)))
    }
    Err(new_error("Could not create user"))
}

#[delete("/users/<key>")]
pub async fn delete(db: &State<DB>, key: &str) -> Result<(Status, Json<User>), Error> {
    if let Ok(Some(deleted)) = db.delete::<User>(key).await {
        return Ok((Status::Ok, Json(deleted)))
    }
    Err(new_error("Could not delete user"))
}

#[put("/users/<key>", data = "<user>")]
pub async fn update(db: &State<DB>, key: &str , user: User) -> Result<Status, Error>{
    if let Ok(Some(_)) = db.update(key, user).await {
       return Ok(Status::Ok)
    }
    Err(new_error("Could not update user"))
}