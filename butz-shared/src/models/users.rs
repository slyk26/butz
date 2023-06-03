use serde::{Deserialize, Serialize};
use rocket::request::{self, Request};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{Status, ContentType};
use surrealdb::sql::Thing;
use crate::models::traits::Identifiable;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub roles: u32,
}

#[derive(Debug)]
pub enum Error {
    TooLarge,
    NoColon,
    InvalidRole,
    Io(std::io::Error),
}

impl Identifiable for User {
    fn get_id(&self) -> &Option<Thing> {
        &self.id
    }

    fn set_id(&mut self, key: &str) {
        let (table, id) = match key.find(':') {
            Some(i) => (&key[..i], &key[(i + 1)..]),
            None => ("", "")
        };
        self.id = Some(Thing::from((table, id)));

    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for User {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let user_ct = ContentType::new("application", "x-user");
        if req.content_type() != Some(&user_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'user' or fallback to default.
        let limit = req.limits().get("user").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);

        let (name, roles) = match string.find(':') {
            Some(i) => (&string[..i], &string[(i + 1)..]),
            None => return Failure((Status::UnprocessableEntity, NoColon)),
        };

        // Parse the roles.
        let roles: u32 = match roles.parse() {
            Ok(roles) => roles,
            Err(_) => return Failure((Status::UnprocessableEntity, InvalidRole)),
        };

        Success(User { id: None, name: name.to_string(), roles })
    }
}
