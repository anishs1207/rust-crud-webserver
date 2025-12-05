// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Uuid,
        book_name -> Varchar,
        author -> Varchar,
    }
}
