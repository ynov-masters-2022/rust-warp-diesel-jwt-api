use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub password: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub password: String,
}
