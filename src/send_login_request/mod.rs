
use std::io::Error;

use login_request_body::{build_anki_sync_header, build_request_body};
use reqwest::Response;
use decompress::decompress_result;

use crate::send_login_request::decompress::LoginResult;

mod decompress;
mod login_request_body;

pub async fn send_login_request(password: String) -> Result<LoginResult, Error> {
    let result = inner_send(password).await.unwrap().bytes();
    let bytes = result.await.unwrap();
    decompress_result(&bytes[..])
}


fn inner_send(password: String) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let body = build_request_body(password);
    let anki_sync_header = build_anki_sync_header();
    let client = reqwest::Client::new();
    client.post("https://sync.ankiweb.net/sync/hostKey")
        .header("anki-sync", anki_sync_header)
        .header("Content-Type", "application/octet-stream")
        .body(body)
        .send()
}