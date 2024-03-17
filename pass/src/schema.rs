// @generated automatically by Diesel CLI.

diesel::table! {
    passwords (id) {
        id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}
