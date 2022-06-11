use crate::schema::users;
use serde::{Deserialize, Serialize};

pub trait UserModel {}

#[derive(Queryable, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub password: String,
}
impl UserModel for User {}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub password: String,
}
impl UserModel for NewUser {}
