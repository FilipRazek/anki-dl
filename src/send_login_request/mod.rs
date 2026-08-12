
use std::io::Error;
use reqwest::Response;
pub use request_result::LoginResult;

mod request_result;
mod request_body;
mod request_headers;


pub async fn send_login_request(user: String, password: String) -> Result<LoginResult, Error> {
    let result = send_http_request(user, password).await.unwrap().bytes();
    let bytes = result.await.unwrap();
    request_result::decompress_result(&bytes[..])
}

fn send_http_request(user: String, password: String) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let body = request_body::build(user, password);
    let anki_sync_header = request_headers::build_anki_sync();
    let client = reqwest::Client::new();
    client.post("https://sync.ankiweb.net/sync/hostKey")
        .header("anki-sync", anki_sync_header)
        .header("Content-Type", "application/octet-stream")
        .body(body)
        .send()
}