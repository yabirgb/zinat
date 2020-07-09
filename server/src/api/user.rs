use crate::db::user::{User, User_profile, NewUserProfileForm};
use crate::api::APIError;
use crate::DbPool;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize)]
struct Register {
  pub username: String,
  pub email: Option<String>,
  pub password: String,
  pub password_confirmation: String,
  pub admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub jwt: String
}



pub fn register_user_action(
  form: &Register, 
  conn:&PgConnection
)->Result<User_profile, APIError>{

  // TODO: Ensure that registrations are open

  // Check that the password match
  if form.password != form.password_confirmation{
    return Err(APIError::err("Password does not match"));
  }

  let user_form = NewUserProfileForm{
    username: form.username,
    email: form.email,
    password_secured: form.password,
    is_admin: false,
    can_login: true
  };

  let user_profile_result = match User_profile::register(conn, &user_form) {
    Ok(profile) => profile,
    Err(e) => return Err(e),
  };

  Ok(user_profile_result)

}

pub async fn register_user(
  pool: web::Data<DbPool>,
  form: web::Json<Register>,
)->Result<HttpResponse, Error>{
  let conn = pool.get().expect("couldn't get db connection from pool");

  let user = web::block(move || register_user_action(&form, &conn))
              .await
              .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
              })?;

  Ok(HttpResponse::Ok().json(user))
}