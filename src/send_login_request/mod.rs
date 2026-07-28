
use std::io::Error;
use reqwest::Response;
use decompress::decompress_result;

use crate::send_login_request::decompress::LoginResult;

mod decompress;
mod request_body;
mod request_headers;

pub async fn send_login_request(password: String) -> Result<LoginResult, Error> {
    let result = send_http_request(password).await.unwrap().bytes();
    let bytes = result.await.unwrap();
    decompress_result(&bytes[..])
}

fn send_http_request(password: String) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let body = request_body::build(password);
    let anki_sync_header = request_headers::build_anki_sync();
    let client = reqwest::Client::new();
    client.post("https://sync.ankiweb.net/sync/hostKey")
        .header("anki-sync", anki_sync_header)
        .header("Content-Type", "application/octet-stream")
        .body(body)
        .send()
}