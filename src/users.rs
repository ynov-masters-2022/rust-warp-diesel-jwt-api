use bcrypt::{hash, DEFAULT_COST};

use crate::db::establish_connection;
use crate::diesel::prelude::*;
use crate::models::{NewUser, User};
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

pub fn create(mut new_user: NewUser) -> User {
  use schema::users;

  let connection = establish_connection();

  new_user.password = hash(new_user.password, DEFAULT_COST).unwrap();

  diesel::insert_into(users::table)
    .values(new_user)
    .get_result(&connection)
    .expect("Failed to save user")
}
