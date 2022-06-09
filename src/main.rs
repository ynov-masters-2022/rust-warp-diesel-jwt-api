use warp_diesel_jwt_api::users;

fn main() {
  let users = users::get_all();

  if users.is_err() {
    panic!("Impossible to get users");
  }

  println!("{:?}", users);
}
