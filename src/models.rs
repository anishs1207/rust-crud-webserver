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

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

// Struct used for inserting a user
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub username: Option<String>,
}
