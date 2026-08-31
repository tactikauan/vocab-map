use std::{collections::BTreeMap, env};

use actix_web::http::header::{HeaderMap, HeaderName};
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

pub fn get_token(headers: &HeaderMap) -> Option<BTreeMap<String, String>> {
    let Some(authorization) = headers.get(HeaderName::from_static("authorization")) else {
        return None;
    };

    let authorization = authorization.to_str().unwrap();

    let token_str = authorization.replace("Bearer ", "");
    return token_str.verify_with_key(&get_key()).unwrap_or(None);
}

pub fn get_key() -> Hmac<Sha256> {
    dotenv().ok();
    let key = env::var("TOKEN_KEY").expect("TOKEN_KEY must be set");
    Hmac::new_from_slice(key.as_bytes()).unwrap()
}

pub fn get_user_id(headers: &HeaderMap) -> Option<i32> {
    let Some(mut token) = get_token(headers) else {
        return None;
    };
    let Some(sub) = token.remove("sub") else {
        return None;
    };
    let Ok(user_id) = sub.parse() else {
        return None;
    };

    return Some(user_id);
}
