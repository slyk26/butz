pub mod users;

#[macro_export]
macro_rules! get_all {
    ($endpoint:literal) => {
        #[get($endpoint)]
        async fn get_all(db: &State<DB>) -> Result<Json<Vec<Model>>, Status> {
            Ok(Json(db.get_all(TABLE)
            .await.map_err(|_| Status::InternalServerError)?))
        }
    }
}

#[macro_export]
macro_rules! get {
    ($endpoint:literal) => {
        #[get($endpoint)]
        async fn get(db: &State<DB>, key: &str) -> Result<Json<Model>, Status> {
            match db.get(key).await {
                Ok(Some(model)) => Ok(Json(model)),
                Ok(None) => Err(Status::NotFound),
                Err(_) => Err(Status::InternalServerError)
            }
        }
    }
}

#[macro_export]
macro_rules! post {
    ($endpoint:literal) => {
        #[post($endpoint, data = "<body>")]
        async fn post(db: &State<DB>, body: Model) -> Result<(Status, Json<Model>), Status> {
            if let Ok(inserted) = db.add(TABLE, body).await {
                return Ok((Status::Created, Json(inserted)))
            }
            Err(Status::InternalServerError)
        }
    }
}

#[macro_export]
macro_rules! delete {
    ($endpoint:literal) => {
        #[delete($endpoint)]
        async fn delete(db: &State<DB>, key: &str) -> Result<(Status, Json<Model>), Status> {
            match db.delete(key).await {
                Ok(Some(model)) => Ok((Status::Ok, Json(model))),
                Ok(None) => Err(Status::NotFound),
                Err(_) => Err(Status::InternalServerError)
            }
        }
    }
}

#[macro_export]
macro_rules! put {
    ($endpoint:literal) => {
        #[put($endpoint, data = "<body>")]
        async fn put(db: &State<DB>, key: &str , body: Model) -> Result<Status, Status>{
            match db.get::<Model>(key).await {
                Ok(Some(_)) => {
                    match db.update(key, body).await {
                        Ok(Some(_)) => Ok(Status::Ok),
                        _ => Err(Status::InternalServerError)
                    }
                }
                _ => Err(Status::NotFound)
            }
        }
    }
}
