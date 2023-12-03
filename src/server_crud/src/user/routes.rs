use crate::user::controller::{handle_get, handle_post};

pub fn user_get(_: Vec<String>, _: String) -> String {
    match handle_get() {
        Ok(value) => value,
        Err(_) => String::from("HTTP/1.1 500 Internal Server Error\r\n\r\n"),
    }
}

pub fn user_post(request: Vec<String>, body: String) -> String {
    match handle_post(request, body) {
        Ok(value) => value,
        Err(value) => {
            let value_string = value.to_string();
            if value_string.starts_with("HTTP/1.1") {
                value_string
            } else {
                String::from("HTTP/1.1 500 Internal Server Error\r\n\r\n")
            }
        }
    }
}
