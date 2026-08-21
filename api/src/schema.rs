// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "language"))]
    pub struct Language;
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    embedding (word) {
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

    user_vocab (user, vocab) {
        user -> Int4,
        vocab -> Int4,
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

diesel::joinable!(user_vocab -> user (user));
diesel::joinable!(user_vocab -> vocab (vocab));

diesel::allow_tables_to_appear_in_same_query!(embedding, user, user_vocab, vocab,);
