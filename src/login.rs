use crate::{
  auth::{self, Role},
  db::establish_connection,
  models::User,
  schema,
};
use bcrypt::verify;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
  pub token: String,
}

pub fn login(request: LoginRequest) -> Result<LoginResponse, String> {
  use schema::users::dsl::*;
  let connection = establish_connection();

  let user = users
    .filter(email.eq(request.email))
    .first::<User>(&connection);

  if user.is_err() {
    return Err("Utilisateur non trouvé".to_string());
  }

  let user = user.unwrap();

  if !verify(request.password, &user.password).unwrap() {
    return Err("Identifiants incorrects".to_string());
  }

  match auth::create_jwt(&user.id.to_string(), &Role::from_str("User")) {
    Ok(token) => Ok(LoginResponse { token }),
    _ => Err("Une erreur est survenue".to_string()),
  }
}
