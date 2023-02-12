// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        cpf -> Varchar,
        password -> Varchar,
        active -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    books,
    users,
);
