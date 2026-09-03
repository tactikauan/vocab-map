use crate::models::User;
use crate::{repository, util};

use jwt::SignWithKey;
use std::collections::BTreeMap;

pub fn auth(username: &str, password: &str) -> Result<String, &'static str> {
    let user = repository::user::get_by_name(username)?;

    Ok(create_token(user)
        .sign_with_key(&util::token::get_key())
        .unwrap())
}

fn create_token(user: User) -> BTreeMap<&'static str, String> {
    let mut claims = BTreeMap::new();
    claims.insert("sub", user.id.to_string());
    claims.insert("name", user.name);
    claims
}
