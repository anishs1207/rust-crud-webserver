// @generated automatically by Diesel CLI.

// defined a table for storing the books
diesel::table! {
    books(id) {
        id -> Int4,
        book_name -> VarChar,
        author -> Varchar,
    }
}
