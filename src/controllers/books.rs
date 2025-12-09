mod middlewares;
use crate::middlewares::{auth::auth_middleware, logger::log_requests};
use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use uuid::Uuid;

mod db;

use argon2::{Argon2, PasswordHasher};
// use rand::rngs::OsRng;

use tokio::task;

use argon2::PasswordVerifier;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;

use crate::db::models::Claims;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use password_hash::SaltString;
use password_hash::rand_core::OsRng;

use crate::db::models::{Book, NewBook, NewUser, UpdateBook, User};
use crate::db::schema::books::dsl::*;

// Type alias for the DB pool
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

// GET /books - gets all the books
async fn get_all_books(State(pool): State<DbPool>) -> impl IntoResponse {
    let result = task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");

        books
            .select(Book::as_select())
            .load::<Book>(&mut conn)
            .expect("Failed to load books")
    })
    .await
    .unwrap();

    (StatusCode::OK, Json(result))
}
