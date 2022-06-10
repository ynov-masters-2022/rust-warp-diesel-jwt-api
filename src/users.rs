use crate::db::establish_connection;
use crate::diesel::prelude::*;
use crate::models::User;
use crate::schema;

pub fn get_all() -> Result<Vec<User>, String> {
  use schema::users::dsl::*;
  let connection = establish_connection();

  if let Ok(users_list) = users.load::<User>(&connection) {
    Ok(users_list)
  } else {
    return Err("Impossible to get all users".to_string());
  }
}

pub fn create(new_user: User) -> User {
  use schema::users;

  let connection = establish_connection();

  diesel::insert_into(users::table)
    .values(new_user)
    .get_result(&connection)
    .expect("Failed to save user")
}
