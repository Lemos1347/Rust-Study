use crate::user::service::User;
use crate::user::temp::MeuObjeto;
use crate::utils::httpresponses::{HTTPContentType, HTTPResponses, JSONResponse};
use std::error::Error;

pub fn handle_get() -> Result<String, Box<dyn Error>> {
    let meu_objeto = MeuObjeto {
        campo1: String::from("valor1"),
        campo2: 123,
    };

    let json = serde_json::to_string(&meu_objeto)?;

    println!("JSON: {}", json);

    Ok(format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        json.len(),
        json
    ))
}

pub fn handle_post(request: Vec<String>, body: String) -> Result<String, String> {
    content_type_must_json(&request)?;

    let user: User = match serde_json::from_str::<User>(&body) {
        Ok(result) => User::new(result.name, result.email, result.password, true),
        _ => return Err(String::from("HTTP/1.1 400 Bad Request\r\n\r\n")),
    };

    println!("User: {:#?}", user);

    match user.create_user() {
        Ok(value) => {
            let json: JSONResponse = JSONResponse { message: value };
            return Ok(format!(
                "{}{}",
                HTTPResponses::Sucess.get_str(),
                HTTPContentType::JSON(serde_json::to_string(&json).unwrap()).get_str()
            ));
        }
        Err(value) => {
            let json = JSONResponse {
                message: value.to_string(),
            };

            Err(format!(
                "{}{}",
                HTTPResponses::BadRequest.get_str(),
                HTTPContentType::JSON(serde_json::to_string(&json).unwrap()).get_str()
            ))
        }
    }
}

fn find_content_type(request: &Vec<String>) -> Result<String, String> {
    for element in request {
        let mut words = element.split_whitespace();
        if words.next().unwrap() == String::from("Content-Type:") {
            return Ok(String::from(words.next().unwrap()));
        }
    }
    Err(String::from("HTTP/1.1 415 Unsupported Media Type\r\n\r\n"))
}

fn content_type_must_json(request: &Vec<String>) -> Result<(), String> {
    let content = find_content_type(request)?;
    if content == "application/json" {
        Ok(())
    } else {
        Err(String::from("HTTP/1.1 415 Unsupported Media Type\r\n\r\n"))
    }
}
