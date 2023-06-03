use std::sync::Arc;
use serde::Serialize;
use serde::de::DeserializeOwned;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{Surreal, Result};
use surrealdb::opt::auth::Root;
use butz_shared::models::traits::Identifiable;
use butz_shared::utils::split_key;

#[derive(Clone)]
pub struct DB {
    client: Arc<Surreal<Client>>,
}

impl DB {
    pub async fn new() -> Result<Self> {
        let bob = Surreal::new::<Ws>("localhost:6969").await.unwrap();

        // TODO CHANGE TO ENV FILE AND IN COMPOSE TOO
        bob.signin(Root {
            username: "root",
            password: "root",
        }).await.unwrap();

        bob.use_ns("butz").use_db("butz").await.unwrap();
        Ok(Self { client: Arc::from(bob) })
    }

    pub async fn get_all<T: Serialize + DeserializeOwned + Send + Sync>(&self, table: &str) -> Result<Vec<T>> {
        self.client.select(table).await
    }

    pub async fn get<T: Serialize + DeserializeOwned + Send + Sync>(&self, key: &str) -> Result<Option<T>> {
         self.client.select(split_key(key)).await
    }

    pub async fn add<T: Serialize + DeserializeOwned + Send + Sync>(&self, table: &str, obj: T) -> Result<T> {
        self.client.create(table).content(obj).await
    }

    pub async fn delete<T: Serialize + DeserializeOwned + Send + Sync>(&self, key: &str) -> Result<Option<T>> {
        self.client.delete(split_key(key)).await
    }

    pub async fn update<T: Serialize + DeserializeOwned + Send + Sync + Identifiable>(&self, key: &str, mut obj: T) -> Result<Option<T>> {
        obj.set_id(key);
        self.client.update(split_key(key)).await
    }
}
