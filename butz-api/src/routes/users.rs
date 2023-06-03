use std::io::ErrorKind;
use rocket::serde::json::Json;
use rocket::State;
use butz_shared::models::User;
use crate::db::DB;

const TABLE: &str = "users";

#[get("/users")]
pub async fn get_all(db: &State<DB>) -> Result<Json<Vec<User>>, std::io::Error> {
    let r = db.get_all(TABLE)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to fetch users"))?;
    Ok(Json(r))
}

#[get("/users/<key>")]
pub async fn get(db: &State<DB>, key: &str) -> Result<Json<User>, std::io::Error> {
    if let Ok(Some(user)) = db.get(key).await {
        return Ok(Json(user))
    }
    Err(std::io::Error::new(ErrorKind::Other, "Could not find user"))
}

#[post("/users", data = "<user>")]
pub async fn add(db: &State<DB>, user: User) -> Result<Json<User>, std::io::Error> {
    match db.add(TABLE, user).await {
        Ok(inserted) => {
            Ok(Json(inserted))
        }
        Err(_) => {
            Err(std::io::Error::new(ErrorKind::Other, "Could not create User"))
        }
    }
}

#[delete("/users/<key>")]
pub async fn delete(db: &State<DB>, key: &str) -> Result<Json<User>, std::io::Error> {
    if let Ok(Some(Some(deleted))) = db.delete::<Option<User>>(key).await {
        return Ok(Json(deleted))
    }
    Err(std::io::Error::new(ErrorKind::Other, "Could not delete User"))
}

#[put("/users/<key>", data = "<user>")]
pub async fn update(db: &State<DB>, key: &str , user: User) -> Result<Json<User>, std::io::Error>{
    if let Ok(Some(updated)) = db.update(key, user).await {
       return Ok(Json(updated))
    }
    Err(std::io::Error::new(ErrorKind::Other, "Could not update User"))
}