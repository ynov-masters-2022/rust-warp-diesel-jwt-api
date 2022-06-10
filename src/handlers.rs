use std::convert::Infallible;

use crate::{models::User, users};

pub async fn list_users() -> Result<impl warp::Reply, Infallible> {
  Ok(warp::reply::json(&users::get_all()))
}

pub async fn create_user(new_user: User) -> Result<impl warp::Reply, Infallible> {
  Ok(warp::reply::json(&users::create(new_user)))
}
