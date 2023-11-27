pub enum HTTPResponses {
    Sucess,
    Created,
    NotFound,
    BadRequest,
    UnsupportedMediaType,
}

impl HTTPResponses {
    pub fn get_str(&self) -> String {
        match self {
            HTTPResponses::Sucess => String::from("HTTP/1.1 200 OK\r\n"),
            HTTPResponses::Created => String::from(""),
            HTTPResponses::NotFound => String::from("HTTP/1.1 404 NOT_FOUND\r\n"),
            HTTPResponses::BadRequest => String::from("HTTP/1.1 400 Bad Request\r\n"),
            HTTPResponses::UnsupportedMediaType => {
                String::from("HTTP/1.1 415 Unsupported Media Type\r\n")
            }
        }
    }
}

pub enum HTTPContentType {
    JSON(String),
}

impl HTTPContentType {
    pub fn get_str(&self) -> String {
        match self {
            HTTPContentType::JSON(content) => {
                format!(
                    "Content-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                )
            }
        }
    }
}
