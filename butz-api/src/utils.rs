use std::io::{Error, ErrorKind};
use rocket::data::{FromData, ToByteUnit};
use rocket::{Data, data, Request};
use rocket::http::{ContentType, Status};
use serde::de::DeserializeOwned;
use crate::models::FromError;

pub fn split_key(key: &str) -> (&str, &str) {
    match key.find(':') {
        Some(i) => (&key[..i], &key[(i + 1)..]),
        None => ("", ""),
    }
}

pub fn new_error(msg: &str) -> Error {
    Error::new(ErrorKind::Other, msg)
}

pub async fn parse<'r, T: FromData<'r, Error = FromError> + DeserializeOwned>(req: &'r Request<'_>, data: Data<'r>, content_type: String, limit_key: &str) -> data::Outcome<'r, T> {
    use FromError::*;
    use rocket::outcome::Outcome::*;

    let user_ct = ContentType::new("application", content_type);
    if req.content_type() != Some(&user_ct) {
        return Forward(data);
    }

    let limit = req.limits().get(limit_key).unwrap_or(256.bytes());

    let string = match data.open(limit).into_string().await {
        Ok(string) if string.is_complete() => string.into_inner(),
        Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
        Err(e) => return Failure((Status::InternalServerError, Io(e))),
    };

    let v = match serde_json::from_str::<T>(string.as_str()) {
        Ok(a) => {a}
        Err(_) => { return Failure((Status::BadRequest, Io(new_error("Could not generate Struct"))))}
    };
    Success(v)
}
