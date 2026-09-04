use anyhow::Result;
use serde::Serialize;

const SYNC_PROTOCOL_VERSION: i32 = 11;

#[derive(Serialize)]
struct LoginAnkiSyncHeader {
    v: i32,
    k: String,
    c: String,
    s: String,
}

pub fn build_anki_sync() -> Result<String> {
    let client_version = format!("{app},{version}", app = "anki-dl", version = "0.1.0");

    let login_header = LoginAnkiSyncHeader {
        v: SYNC_PROTOCOL_VERSION,
        c: client_version,
        k: String::from(""),
        s: String::from("test_session"),
    };
    Ok(serde_json::to_string(&login_header)?)
}
