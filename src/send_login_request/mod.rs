use reqwest::Response;
pub use request_result::LoginResult;
use entities::UserCredentials;
use anyhow::Result;

mod request_result;
mod request_body;
mod request_headers;
mod entities;


pub async fn send_login_request(user: String, password: String) -> Result<LoginResult> {
    let result = send_http_request(user, password).await?.bytes();
    let bytes = result.await?;
    LoginResult::decompress_result(&bytes[..])
}

async fn send_http_request(user: String, password: String) -> Result<Response> {
    let body = request_body::build(    UserCredentials {
        user: user,
        password: password
    })?;
    let anki_sync_header = request_headers::build_anki_sync();
    let client = reqwest::Client::new();
    Ok(client.post("https://sync.ankiweb.net/sync/hostKey")
        .header("anki-sync", anki_sync_header)
        .header("Content-Type", "application/octet-stream")
        .body(body)
        .send()
        .await?)
}
