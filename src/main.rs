use warp::Filter;
use warp_diesel_jwt_api::{error, routes};

#[tokio::main]
async fn main() {
  let routes = routes::routes().recover(error::handle_rejection);

  warp::serve(routes).run(([127, 0, 0, 1], 3500)).await;
}
