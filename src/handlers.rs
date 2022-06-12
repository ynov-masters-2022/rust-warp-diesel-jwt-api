use std::convert::Infallible;

use crate::{
  login::{login, LoginRequest},
  models::NewUser,
  users,
};

pub async fn list_users(_uid: String) -> Result<impl warp::Reply, Infallible> {
  Ok(warp::reply::json(&users::get_all().unwrap()))
}

pub async fn create_user(new_user: NewUser) -> Result<impl warp::Reply, Infallible> {
  Ok(warp::reply::json(&users::create(new_user)))
}

pub async fn authenticate(body: LoginRequest) -> Result<impl warp::Reply, Infallible> {
  Ok(warp::reply::json(&login(body).unwrap()))
}
