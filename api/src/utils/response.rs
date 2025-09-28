use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseMessage<'a> {
    message: &'a str,
}

pub fn message(message: &str) -> ResponseMessage {
    ResponseMessage { message }
}
