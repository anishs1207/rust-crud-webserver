use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// BOOKS TABLE:
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: Uuid,
    pub book_name: String,
    pub author: String,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::books)]
pub struct NewBook {
    pub book_name: String,
    pub author: String,
}
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::books)]
pub struct UpdateBook {
    pub book_name: Option<String>,
    pub author: Option<String>,
}

// USER TABLE:

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

// #[derive(Insertable, Deserialize)]
// #[diesel(table_name = users)]
// pub struct NewUser {
//     pub username: String,
//     pub email: String,
// }
