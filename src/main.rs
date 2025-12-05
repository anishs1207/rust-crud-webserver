use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use tokio::task;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use crate::models::Book;
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
async fn get_book_by_id(State(pool): State<DbPool>, Path(book_id): Path<i32>) -> impl IntoResponse {
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

    // defined the router for the routing
    let app = Router::new()
        .route("/", get(|| async { "Works" }))
        .route("/books", get(get_all_books))
        .route(
            "/books/{id}",
            get(get_book_by_id), // .patch(update_book_by_id)
                                 // .delete(delete_book_by_id),
        )
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
