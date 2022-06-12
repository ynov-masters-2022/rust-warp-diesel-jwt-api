use warp::Rejection;

#[macro_use]
extern crate diesel;

pub mod auth;
pub mod db;
pub mod error;
pub mod handlers;
pub mod login;
pub mod models;
pub mod routes;
pub mod schema;
pub mod users;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
