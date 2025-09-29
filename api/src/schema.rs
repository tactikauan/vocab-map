// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "language"))]
    pub struct Language;
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    embeddings (word) {
        word -> Varchar,
        vector -> Vector,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    user (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;
    use super::sql_types::Language;

    vocab (id) {
        id -> Int4,
        word -> Varchar,
        lang -> Language,
    }
}

diesel::allow_tables_to_appear_in_same_query!(embeddings, user, vocab,);
