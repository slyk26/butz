pub mod users;

#[macro_export]
macro_rules! get_all {
    ($endpoint:literal) => {
        #[get($endpoint)]
        pub async fn get_all(db: &State<DB>) -> Result<Json<Vec<Model>>, Error> {
            Ok(Json(db.get_all(TABLE)
            .await.map_err(|_| new_error("Could not execute get_all"))?))
        }
    }
}

#[macro_export]
macro_rules! get {
    ($endpoint:literal) => {
        #[get($endpoint)]
        pub async fn get(db: &State<DB>, key: &str) -> Result<Json<Model>, Error> {
            if let Ok(Some(model)) = db.get(key).await {
                return Ok(Json(model))
            }
            Err(new_error("Could not execute get"))
        }
    }
}

#[macro_export]
macro_rules! post {
    ($endpoint:literal) => {
        #[post($endpoint, data = "<body>")]
        pub async fn add(db: &State<DB>, body: Model) -> Result<(Status, Json<Model>), Error> {
            if let Ok(inserted) = db.add(TABLE, body).await {
                return Ok((Status::Created, Json(inserted)))
            }
            Err(new_error("Could not execute create"))
        }
    }
}

#[macro_export]
macro_rules! delete {
    ($endpoint:literal) => {
        #[delete($endpoint)]
        pub async fn delete(db: &State<DB>, key: &str) -> Result<(Status, Json<Model>), Error> {
            if let Ok(Some(deleted)) = db.delete::<Model>(key).await {
                return Ok((Status::Ok, Json(deleted)))
            }
            Err(new_error("Could not execute delete"))
        }
    }
}

#[macro_export]
macro_rules! put {
    ($endpoint:literal) => {
        #[put($endpoint, data = "<body>")]
        pub async fn update(db: &State<DB>, key: &str , body: Model) -> Result<Status, Error>{
            if let Ok(Some(_)) = db.update(key, body).await {
                return Ok(Status::Ok)
            }
            Err(new_error("Could not execute update"))
        }
    }
}
