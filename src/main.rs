use warp_diesel_jwt_api::routes;

#[tokio::main]
async fn main() {
  let users_routes = routes::users_routes();

  warp::serve(users_routes).run(([127, 0, 0, 1], 3500)).await;
}
