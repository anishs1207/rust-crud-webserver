// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        book_name -> Varchar,
        author -> Varchar,
    }
}
