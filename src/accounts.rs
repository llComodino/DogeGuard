// Account Handler
// Handles all account related requests
// -------------------------------------------

use crate::db::models::account::Account;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn get_account_by_id(
    pool: web::Data<crate::db::Pool>,
    id: web::Path<i32>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let account = web::block(move || Account::find_by_id(&conn, id.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let response = AccountResponse {
        id: account.id,
        username: account.username,
        email: account.email,
        password: account.password,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_account_by_username(
    pool: web::Data<crate::db::Pool>,
    username: web::Path<String>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let account = web::block(move || Account::find_by_username(&conn, username.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let response = AccountResponse {
        id: account.id,
        username: account.username,
        email: account.email,
        password: account.password,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_account_by_email(
    pool: web::Data<crate::db::Pool>,
    email: web::Path<String>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let account = web::block(move || Account::find_by_email(&conn, email.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let response = AccountResponse {
        id: account.id,
        username: account.username,
        email: account.email,
        password: account.password,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_account(
    pool: web::Data<crate::db::Pool>,
    account: web::Json<Account>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let account = web::block(move || Account::create(&conn, account.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let response = AccountResponse {
        id: account.id,
        username: account.username,
        email: account.email,
        password: account.password,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn update_account(
    pool: web::Data<crate::db::Pool>,
    account: web::Json<Account>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let account = web::block(move || Account::update(&conn, account.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let response = AccountResponse {
        id: account.id,
        username: account.username,
        email: account.email,
        password: account.password,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_account(
    pool: web::Data<crate::db::Pool>,
    id: web::Path<i32>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    web::block(move || Account::delete(&conn, id.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}
