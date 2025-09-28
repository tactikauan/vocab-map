// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    embeddings (word) {
        word -> Varchar,
        vector -> Vector,
    }
}
