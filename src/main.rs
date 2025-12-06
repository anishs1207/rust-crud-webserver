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

pub mod models;
pub mod schema;

use argon2::{Argon2, PasswordHasher};
// use rand::rngs::OsRng;

use tokio::task;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use password_hash::SaltString;
use password_hash::rand_core::OsRng;

use crate::models::{Book, NewBook, NewUser, UpdateBook, User};
use crate::schema::books::dsl::*;

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

// GET /books/{id}
async fn get_book_by_id(
    State(pool): State<DbPool>,
    Path(book_id): Path<Uuid>,
) -> impl IntoResponse {
    // Spawn a blocking task for Diesel DB access
    let result = task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");

        books
            .filter(id.eq(book_id))
            .select(Book::as_select())
            .first::<Book>(&mut conn)
            .optional() // returns Ok(Some(book)) or Ok(None) instead of error if not found
    })
    .await
    .unwrap(); // unwrap JoinHandle

    match result {
        Ok(Some(book)) => Json(book).into_response(),
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "Book not found").into_response(),
        Err(err) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        )
            .into_response(),
    }
}

// POST /books - add a new book
async fn add_book(State(pool): State<DbPool>, Json(payload): Json<NewBook>) -> impl IntoResponse {
    let result = task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");

        diesel::insert_into(books)
            .values(&payload)
            .returning(Book::as_returning())
            .get_result::<Book>(&mut conn)
    })
    .await
    .unwrap();

    match result {
        Ok(book) => (StatusCode::CREATED, Json(book)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to insert book: {}", err),
        )
            .into_response(),
    }
}

// PATCH /books/{id}
#[axum::debug_handler]
async fn update_book_by_id(
    State(pool): State<DbPool>,
    Path(book_id): Path<Uuid>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, (StatusCode, String)> {
    // Run DB operation in blocking thread
    let result = task::spawn_blocking(move || {
        let mut conn = pool.get().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DB pool error".to_string(),
            )
        })?;

        diesel::update(books.filter(id.eq(book_id)))
            .set(&payload)
            .get_result::<Book>(&mut conn)
            .optional()
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("DB error: {}", err),
                )
            })
    })
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Join error: {}", err),
        )
    })?; // <- unwrap outer Result

    // Now `result` is Option<Book>, safe to match
    match result {
        Ok(Some(book)) => Ok(Json(book)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Book not found".into())),
        Err(e) => Err(e),
    }
}

// DELETE /books/{id}
#[axum::debug_handler]
async fn delete_book_by_id(
    State(pool): State<DbPool>,
    Path(book_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = task::spawn_blocking(move || {
        let mut conn = pool.get().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DB pool error".to_string(),
            )
        })?;

        diesel::delete(books.filter(id.eq(book_id)))
            .execute(&mut conn)
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("DB error: {}", err),
                )
            })
    })
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Join error: {}", err),
        )
    })?;

    match result {
        Ok(count) if count > 0 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err((StatusCode::NOT_FOUND, "Book not found".into())),
        Err(e) => Err(e),
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

// POST /register
pub async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    println!(" Username {}", payload.username);
    println!(" Email: {}", payload.email);
    println!("Password: {}", payload.password);
    println!("Hashed Passwod: {}", hashed_password);

    let new_user = NewUser {
        username: payload.username,
        email: payload.email,
        password: hashed_password,
    };

    let result = task::spawn_blocking(move || {
    let mut conn = pool.get().unwrap();

    diesel::insert_into(crate::schema::users::dsl::users)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result::<User>(&mut conn)
    })
    .await
    .unwrap(); // JoinHandle< Result<User, diesel::Error> >


    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        )
            .into_response(),
    }

}

// POST /login
// pub async fn login(
//     State(pool): State<DbPool>,
//     Json(payload): Json<LoginPayload>,
// ) -> impl IntoResponse {
//     let user_result = tokio::task::spawn_blocking(move || {
//         let mut conn = pool.get().unwrap();
//         crate::schema::users::dsl::users
//             .filter(crate::schema::users::dsl::email.eq(&payload.email))
//             .first::<User>(&mut conn)
//             .optional()
//     })
//     .await
//     .unwrap();

//     match user_result {
//         Ok(Some(user)) => {
//             if verify_encoded(&user.password, payload.password.as_bytes()).unwrap() {
//                 let claims = Claims {
//                     sub: user.id.to_string(),
//                     exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
//                 };
//                 let token = encode(
//                     &Header::default(),
//                     &claims,
//                     &EncodingKey::from_secret("secret".as_ref()),
//                 )
//                 .unwrap();
//                 (StatusCode::OK, Json(serde_json::json!({ "token": token }))).into_response()
//             } else {
//                 (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
//             }
//         }
//         Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
//         Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response(),
//     }
// }

// for the protected routes added here:
// let protected_books = Router::new()
//     .route("/books", get(get_all_books).post(add_book))
//     .route(
//         "/books/{id}",
//         get(get_book_by_id)
//             .patch(update_book_by_id)
//             .delete(delete_book_by_id),
//     )
//     .layer(axum::middleware::from_fn(auth_middleware));

// let app = Router::new()
//     .route("/", get(|| async { "Works" }))
//     .route("/register", register)
//     .route("/login", login)
//     .merge(protected_books)
//     .layer(axum::middleware::from_fn(log_requests))
//     .with_state(pool);

// main entry point for the function:
#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be present");

    println!("DATABASE_URL {}", database_url);

    // only creates one PostgreSQL connection, each time estabvkish is called new TCP connection to DB
    // so each request => creares seperate DB (connection slow & expensive)
    //  PgConnection::establish(&database_url)
    //     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to created Pool");

    let protected_books = Router::new()
        .route("/books", get(get_all_books).post(add_book))
        .route(
            "/books/{id}",
            get(get_book_by_id)
                .patch(update_book_by_id)
                .delete(delete_book_by_id),
        )
        .layer(axum::middleware::from_fn(auth_middleware));

    // defined the router for the routing
    let app = Router::new()
        .route("/", get(|| async { "Works" }))
        .route("/register", post(register))
        // .route("/login", login)
        .merge(protected_books)
        .layer(axum::middleware::from_fn(log_requests))
        .with_state(pool);

    println!("Server running on http://localhost:3000");

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}
