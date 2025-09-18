use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        db_user, db_password, db_host, db_name
    );
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
