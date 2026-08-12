use anyhow::Result;
use serde::Serialize;

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
        v: 11,
        c: client_version,
        k: String::from(""),
        s: String::from("test_session"),
    };
    Ok(serde_json::to_string(&login_header)?)
}
