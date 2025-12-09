// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Uuid,
        book_name -> Varchar,
        author -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(books, users,);
