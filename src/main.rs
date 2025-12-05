use axum::{
    Router,
    extract::{Json, Path, State},
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

//     use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;



use tokio::task;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2::{Pool};
use diesel::r2d2::ConnectionManager;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

use crate::schema::books::dsl::*;


use tokio::task;
use diesel::prelude::*;

// GET /books - gets all the books
async fn get_all_books(
    State(pool): State<DbPool>
) -> Json<Vec<Book>> {

    // Run Diesel (blocking) inside a spawned blocking thread
    let books = task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");

        use crate::schema::books::dsl::*;

        books
            .select(Book::as_select())
            .load::<Book>(&mut conn)
            .expect("Failed to load books")
    })
    .await
    .unwrap();

    Json(books)
}


// GET /books/{id}
// async fn get_book_by_id(State(db): State<Db>, Path(id): Path<u32>) -> impl IntoResponse {
//     let connection= &mut establish_connection();

//     let books = books.find(id).select(Post::as_select).first(connection).optional();

//     match books {
//         Ok()
//     }

//     let books = db.lock().unwrap();
//     if let Some(book) = books.iter().find(|b| b.id == id) {
//         Json(book.clone()).into_response()
//     } else {
//         (axum::http::StatusCode::NOT_FOUND, "Not Found").into_response()
//     }
// }

// // POST /books
// async fn add_book(State(db): State<Db>, Json(payload): Json<CreateBook>) -> impl IntoResponse {
//     let mut books = db.lock().unwrap();

//     // FIXED: avoids duplicate IDs after deletes
//     let new_id = books.last().map(|b| b.id + 1).unwrap_or(1);

//     let book = Book {
//         id: new_id,
//         book_name: payload.book_name,
//         author: payload.author,
//     };

//     books.push(book.clone());
//     Json(book)
// }

// // PATCH /books/{id}
// async fn update_book_by_id(
//     State(db): State<Db>,
//     Path(id): Path<u32>,
//     Json(payload): Json<UpdateBook>,
// ) -> impl IntoResponse {
//     let mut books = db.lock().unwrap();

//     if let Some(book) = books.iter_mut().find(|b| b.id == id) {
//         if let Some(name) = payload.book_name {
//             book.book_name = name;
//         }
//         if let Some(author) = payload.author {
//             book.author = author;
//         }
//         return Json(book.clone()).into_response();
//     }

//     (axum::http::StatusCode::NOT_FOUND, "Not Found").into_response()
// }

// // DELETE /books/{id}
// async fn delete_book_by_id(State(db): State<Db>, Path(id): Path<u32>) -> impl IntoResponse {
//     let mut books = db.lock().unwrap();
//     books.retain(|b| b.id != id);
//     "Deleted"
// }

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
   
}

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
    .expect("Failed to created Pool")


 
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

    let app = Router::new()
        .route("/", get(|| async { "Works" }))
        .route("/books", get(get_all_books).post(add_book))
        .route(
            "/books/{id}", // <-- FIXED FOR AXUM 0.7
            get(get_book_by_id)
                .patch(update_book_by_id)
                .delete(delete_book_by_id),
        )
        .with_state(db);

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}
