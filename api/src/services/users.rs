use crate::models::User;
use crate::schema::user;
use crate::{db, utils};

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use jwt::SignWithKey;
use std::collections::BTreeMap;

pub fn auth(username: &str, password: &str) -> Result<String, &'static str> {
    let connection = &mut db::establish_connection();

    let Ok(user_entity) = user::table
        .filter(user::name.eq(username))
        .select(User::as_select())
        .first(connection)
    else {
        return Err("User not found");
    };

    Ok(create_token(user_entity)
        .sign_with_key(&utils::token::get_key())
        .unwrap())
}

fn create_token(user: User) -> BTreeMap<&'static str, String> {
    let mut claims = BTreeMap::new();
    claims.insert("sub", user.id.to_string());
    claims.insert("name", user.name);
    claims
}
