use crate::db::establish_connection;
use crate::diesel::prelude::*;
use crate::models::User;
use crate::schema;

pub fn get_all() -> Result<Vec<User>, diesel::result::Error> {
  use schema::users::dsl::*;

  let connection = establish_connection();
  users.load::<User>(&connection)
}
