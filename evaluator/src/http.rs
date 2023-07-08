use std::collections::HashMap;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt};

use std::io::Result;
use std::io::{Error, ErrorKind};
use std::str;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub content: Option<String>,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub code: String,
    pub reason_phrase: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

fn http_error(message: &str) -> Error {
    Error::new(ErrorKind::Other, message)
}

pub fn create_http_response(resp: &HttpResponse) -> String {
    let mut result = format!("HTTP/1.1 {} {}\r\n", resp.code, resp.reason_phrase);

    for (name, value) in &resp.headers {
        result.push_str(&format!("{}: {}\r\n", name, value));
    }
    result.push_str("\r\n");

    if let Some(content) = &resp.body {
        result.push_str(content)
    }
    result
}

pub async fn parse_http_request(stream: &mut (impl AsyncBufRead + AsyncRead + Unpin)) -> Result<HttpRequest> {
    let mut buffer = String::new();
    stream.read_line(&mut buffer).await?;
    let mut request_line = buffer.split_whitespace();

    let method = request_line.next().ok_or(http_error("Missing HTTP method"))?.to_string();
    let path = request_line.next().ok_or(http_error("Missing path to resource"))?.to_string();
    let http_version = request_line.next().ok_or(http_error("Missing HTTP version"))?;

    if http_version != "HTTP/1.1" {
        return Err(Error::new(ErrorKind::Other,
                              "Only HTTP/1.1 is supported"));
    }

    // TODO: headers might have multiple of the same name.
    let mut headers = HashMap::new();
    loop {
        buffer.clear();
        stream.read_line(&mut buffer).await?;
        if buffer == "\r\n"  || buffer == "\n" {
            break;
        }
        buffer.pop();
        let mut splitted = buffer.split(':');
        let name = splitted.next().ok_or(http_error("Missing header name"))?.to_lowercase();
        let value = splitted.next().ok_or(http_error("Missing header value"))?;
        // TODO: Handle if it exists already
        headers.insert(name, value.trim().to_string());
    }

    let content = match headers.get("content-length") {
        Some(len) => {
            let len: usize = len.parse().unwrap();
            let mut buffer = vec![0u8; len];
            stream.read_exact(&mut buffer).await?;
            Some(str::from_utf8(&buffer).unwrap().to_string())
        }
        None => None,
    };
    Ok(HttpRequest { method, path, headers, content  })
}

impl HttpRequest {
    pub fn content_type(&self) -> Option<&String> {
        self.headers.get("content-type")
    }

    pub fn content_length(&self) -> Option<usize> {
        let content_length = self.headers.get("content-length");
        content_length.map(|ls| str::parse(ls).expect("content-length header must be number"))
    }
}

impl HttpResponse {
    pub fn new(code: i32, reason_phrase: &str, headers: Vec<(String, String)>, body: String) {
        todo!()        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_req_parsing() {
        let mut request =
r#"POST / HTTP/1.1
Content-length: 10
Content-type: text

Ahoj svete"#.as_bytes();
        let result = parse_http_request(&mut request).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.method, "POST");
        assert_eq!(response.path, "/");
        assert_eq!(response.headers.get("content-length").unwrap(), "10");
        assert_eq!(response.headers.get("content-type").unwrap(), "text");
        assert!(response.content.is_some());
        assert_eq!(response.content.unwrap(), "Ahoj svete");
    }

    #[tokio::test]
    async fn test_req_empty_content() {
        let mut request =
r#"GET / HTTP/1.1

"#.as_bytes();
        let result = parse_http_request(&mut request).await;
        assert!(result.is_ok(), "{:?}", result.unwrap_err());
        let response = result.unwrap();
        assert_eq!(response.method, "GET");
        assert_eq!(response.path, "/");
        assert_eq!(response.headers.len(), 0);
        assert!(response.content.is_none());
    }

    #[tokio::test]
    async fn test_req_empty_content_with_headers() {
        let mut request =
r#"GET / HTTP/1.1
User-Agent:   Mozzila5/0  Safari124

"#.as_bytes();
        let result = parse_http_request(&mut request).await;
        assert!(result.is_ok(), "{:?}", result.unwrap_err());
        let response = result.unwrap();
        assert_eq!(response.method, "GET");
        assert_eq!(response.path, "/");
        assert_eq!(response.headers.get("user-agent").unwrap(), "Mozzila5/0  Safari124");
        assert!(response.content.is_none());
    }

    #[tokio::test]
    async fn test_req_no_empty_line() {
        let mut request =
r#"GET / HTTP/1.1
User-Agent:   Mozzila5/0  Safari124
"#.as_bytes();
        let result = parse_http_request(&mut request).await;
        assert!(result.is_err(), "{:?}", result.unwrap());
    }

    #[test]
    fn test_http_create_response() {
        let response = HttpResponse{
            code: "200".to_string(),
            reason_phrase: "OK".to_string(),
            headers: vec![("Content-Length".to_string(), "11".to_string())],
            body: Some("Hello World".to_string())
        };
        let result = create_http_response(&response);
        assert_eq!(result, "HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nHello World");
    }
}
