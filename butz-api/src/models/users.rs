use rocket::request::Request;
use rocket::data::{self, Data, FromData};
use rocket::serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;
use crate::utils::{parse, split_key};
extern crate butz_macros;
use crate::models::Model;

#[derive(Debug, Serialize, Deserialize, Model)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub roles: u32,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for User {
    type Error = std::io::Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        parse(req, data, String::from("x-user"), "user").await
    }
}
