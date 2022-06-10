use warp::Filter;

use crate::{handlers, models::User};

pub fn users_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  users_list().or(user_create())
}

fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn users_list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path("users")
    .and(warp::get())
    .and_then(handlers::list_users)
}

fn user_create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path("create")
    .and(warp::post())
    .and(json_body())
    .and_then(handlers::create_user)
}
