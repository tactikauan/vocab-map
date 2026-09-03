use crate::db;
use crate::models::User;
use crate::schema::user;

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};

pub fn get_by_name(name: &str) -> Result<User, &'static str> {
    let connection = &mut db::establish_connection();

    user::table
        .filter(user::name.eq(name))
        .select(User::as_select())
        .first(connection)
        .or(Err("User not found"))
}
