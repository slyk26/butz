mod users;
mod from_error;

use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::sql::Thing;
pub use users::*;
pub use from_error::*;
use butz_macros::Model;

pub trait Model: Serialize + DeserializeOwned + Send + Sync {
    fn get_id(&self) -> &Option<Thing>;
    fn set_id(&mut self, key: &str);
}