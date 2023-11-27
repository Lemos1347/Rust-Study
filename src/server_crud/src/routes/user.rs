use super::httpresponses::{HTTPContentType, HTTPResponses};
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct MeuObjeto {
    campo1: String,
    campo2: i32,
}

#[derive(Serialize, Deserialize)]
struct JSONResponse {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    password: String,
}

impl User {
    fn _new(name: String, email: String, password: String) -> User {
        User {
            name,
            email,
            password,
        }
    }

    fn create_user(&self) -> Result<String, Box<dyn Error>> {
        // Estabelecer conexão com o banco de dados
        let host = env::var("DB_HOST")?;
        let port = env::var("DB_PORT")?;
        let user = env::var("DB_USER")?;
        let password = env::var("DB_PASSWORD")?;
        let db_name = env::var("DB_NAME")?;

        let params =
            format!("host={host} port={port} user={user} dbname={db_name} password={password}");

        let mut client = Client::connect(&params, NoTls)?;

        // Executar a query de inserção
        let query =
            "INSERT INTO \"User\" (\"username\", \"password\", \"email\") VALUES ($1, $2, $3)";
        client.execute(query, &[&self.name, &self.email, &self.password])?;

        Ok(String::from("User created with sucess!"))
    }
}

// ----------------------------------------------------------------------------------
// ------------------------------------- ROUTES -------------------------------------
// ----------------------------------------------------------------------------------
// GET
pub fn handle_get(_: Vec<String>, _: String) -> String {
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

// POST
pub fn handle_post(request: Vec<String>, body: String) -> String {
    let content_type = find_content_type(&request);

    match content_type {
        Ok(content) if content == "application/json" => (),
        _ => return String::from("HTTP/1.1 415 Unsupported Media Type\r\n\r\n"),
    }

    let user: User = match serde_json::from_str(&body) {
        Ok(result) => result,
        _ => return String::from("HTTP/1.1 400 Bad Request\r\n\r\n"),
    };

    println!("User: {:#?}", user);

    match user.create_user() {
        Ok(value) => {
            let json: JSONResponse = JSONResponse { message: value };
            return format!(
                "{}{}",
                HTTPResponses::Sucess.get_str(),
                HTTPContentType::JSON(serde_json::to_string(&json).unwrap()).get_str()
            );
        }
        Err(value) => {
            let json = JSONResponse {
                message: value.to_string(),
            };
            return format!(
                "{}{}",
                HTTPResponses::BadRequest.get_str(),
                HTTPContentType::JSON(serde_json::to_string(&json).unwrap()).get_str()
            );
        }
    }

    //  String::from("HTTP/1.1 200 OK\r\n\r\n")
}
// ---------------------------------------------------------------------------------
// ------------------------------------- UTILS -------------------------------------
// ---------------------------------------------------------------------------------

fn find_content_type(request: &Vec<String>) -> Result<String, String> {
    for element in request {
        let mut words = element.split_whitespace();
        if words.next().unwrap() == String::from("Content-Type:") {
            return Ok(String::from(words.next().unwrap()));
        }
    }
    Err(String::from("Wrong content type"))
}
