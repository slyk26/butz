use std::io::Error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::db::DB;
use crate::*;
use crate::models::User;
use crate::utils::new_error;

const TABLE: &str = "users";
type Model = User;

get_all!("/users");

get!("/users/<key>");

post!("/users");

delete!("/users/<key>");

put!("/users/<key>");