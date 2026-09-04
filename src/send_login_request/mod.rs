use anyhow::Result;
pub use entities::UserCredentials;
pub use request_result::LoginResult;
use reqwest::Response;

mod entities;
mod request_body;
mod request_headers;
mod request_result;

pub async fn send_login_request(credentials: UserCredentials) -> Result<LoginResult> {
    let response = send_http_request(credentials).await?;
    let bytes = response.bytes().await?;
    LoginResult::decompress_result(&bytes[..])
}

async fn send_http_request(credentials: UserCredentials) -> Result<Response> {
    let body = request_body::build(credentials)?;
    let anki_sync_header = request_headers::build_anki_sync()?;
    let client = reqwest::Client::new();
    Ok(client
        .post("https://sync.ankiweb.net/sync/hostKey")
        .header("anki-sync", anki_sync_header)
        .header("Content-Type", "application/octet-stream")
        .body(body)
        .send()
        .await?)
}
