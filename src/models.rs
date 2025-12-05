use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
