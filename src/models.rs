use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, PartialEq, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub email: String,
  pub password: String,
}
