
use login_request_body::{build_anki_sync_header, build_request_body};
use reqwest::Response;

mod login_request_body;

pub fn send_login_request(password: String) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let body = build_request_body(password);
    let anki_sync_header = build_anki_sync_header();
    let client = reqwest::Client::new();
    client.post("https://sync.ankiweb.net/sync/hostKey")
        .header("anki-sync", anki_sync_header)
        .header("Content-Type", "application/octet-stream")
        .body(body)
        .send()
}
