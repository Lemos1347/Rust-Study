use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String, hash_pass: bool) -> User {
        match hash_pass {
            true => {
                let salt = SaltString::generate(&mut OsRng);
                let argon2 = Argon2::default();
                let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
                User {
                    name,
                    email,
                    password: password_hash.to_string(),
                }
            }
            false => User {
                name,
                email,
                password,
            },
        }
    }

    pub fn create_user(&self) -> Result<String, Box<dyn Error>> {
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
            "INSERT INTO \"User\" (\"username\", \"email\", \"password\") VALUES ($1, $2, $3)";
        client.execute(query, &[&self.name, &self.email, &self.password])?;

        Ok(String::from("User created with sucess!"))
    }
}
