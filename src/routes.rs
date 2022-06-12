use serde::de::DeserializeOwned;
use warp::Filter;

use crate::{
  auth::{with_auth, Role},
  handlers,
  login::LoginRequest,
  models::NewUser,
};

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  login().or(users_routes())
}

pub fn users_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("users").and(users_list().or(user_create()))
}

fn json_body<T: std::marker::Send + DeserializeOwned>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn users_list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path::end()
    .and(warp::get())
    .and(with_auth(Role::User))
    .and_then(handlers::list_users)
}

fn user_create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path::end()
    .and(warp::post())
    .and(json_body::<NewUser>())
    .and_then(handlers::create_user)
}

fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("login")
    .and(warp::post())
    .and(json_body::<LoginRequest>())
    .and_then(handlers::authenticate)
}
