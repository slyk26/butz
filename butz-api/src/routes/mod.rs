pub mod users;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::db::DB;
use crate::models::Model;

async fn get_all<T: Model>(db: &State<DB>, table: &str) -> Result<Json<Vec<T>>, Status> {
    Ok(Json(db.get_all::<T>(table)
        .await.map_err(|_| Status::InternalServerError)?))
}

async fn get<T: Model>(db: &State<DB>, key: &str) -> Result<Json<T>, Status> {
    match db.get(key).await {
        Ok(Some(model)) => Ok(Json(model)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError)
    }
}

async fn post<T: Model>(db: &State<DB>, body: T, table: &str) -> Result<(Status, Json<T>), Status> {
    if let Ok(inserted) = db.add(table, body).await {
        return Ok((Status::Created, Json(inserted)));
    }
    Err(Status::InternalServerError)
}

async fn delete<T: Model>(db: &State<DB>, key: &str) -> Result<(Status, Json<T>), Status> {
    match db.delete::<T>(key).await {
        Ok(Some(model)) => Ok((Status::Ok, Json(model))),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError)
    }
}

async fn put<T: Model>(db: &State<DB>, key: &str, body: T) -> Result<Status, Status> {
    match db.get::<T>(key).await {
        Ok(Some(_)) => {
            match db.update::<T>(key, body).await {
                Ok(Some(_)) => Ok(Status::Ok),
                _ => Err(Status::InternalServerError)
            }
        }
        _ => Err(Status::NotFound)
    }
}