use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MeuObjeto {
    campo1: String,
    campo2: i32,
}

pub fn handle_get(_: Vec<String>) -> String {
    let meu_objeto = MeuObjeto {
        campo1: String::from("valor1"),
        campo2: 123,
    };

    let json = serde_json::to_string(&meu_objeto).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        json.len(),
        json
    );

    response
    //  "HTTP/1.1 200 OK\r\n\r\n"
}

pub fn handle_post(request: Vec<String>) -> String {
    let content_type = find_content_type(&request);

    match content_type {
        Ok(content) if content == "application/json" => (),
        _ => return String::from("HTTP/1.1 415 Unsupported Media Type\r\n\r\n"),
    }

    // if response.len() > 0 {
    //    return response
    // };

    //  println!("Content-type: {content_type}");

    String::from("HTTP/1.1 200 OK\r\n\r\n")
}

fn find_content_type(request: &Vec<String>) -> Result<String, String> {
    for element in request {
        let mut words = element.split_whitespace();
        if words.next().unwrap() == String::from("Content-Type:") {
            return Ok(String::from(words.next().unwrap()));
        }
    }
    Err(String::from("Wrong content type"))
}
