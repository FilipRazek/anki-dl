use std::println;
use std::env;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use zstd::{Decoder, stream::write::Encoder};

fn main() {
    trpl::block_on(login());
}
#[derive(Serialize)]
struct LoginAnkiSyncHeader {
    v: i32,
    k: String,
    c: String,
    s: String
}

#[derive(Serialize)]
struct LoginBody {
    u: String,
    p: String
}

#[derive(Debug, Deserialize)]
struct LoginResult {
    key: String,
} 

async fn login() {

    /*
    POST https://sync.ankiweb.net/sync/hostKey
        Content-Type: application/octet-stream
        anki-sync: {"v":11,"k":"","c":"anki,2.x.x","s":"<any-session-id>"}
        <body = zstd( {"u":"<user>","p":"<pass>"} )>

     */
    let client_version = format!(
            "{app},{version}",
            app = "anki-dl",
            version = "0.1.0"
        );

    dotenv().ok();
    let password = env::var("ANKI_PASSWORD").unwrap();

    let login_header = LoginAnkiSyncHeader {
        v: 11,
        c: client_version,
        k: String::from(""),
        s: String::from("test_session")
    };

    let body = build_request_body(password);

    let client = reqwest::Client::new();

    let res = client.post("https://sync.ankiweb.net/sync/hostKey")
        .header("anki-sync", serde_json::to_string(&login_header).unwrap())
        .header("Content-Type", "application/octet-stream")
        .body(body)
        .send()
        .await
        .unwrap();

    let bytes = res.bytes().await.unwrap();
    let decoder = Decoder::new(&bytes[..]).unwrap();
    let data: LoginResult = serde_json::from_reader(decoder).unwrap();
    println!("{}", data.key);
}

fn build_request_body(password: String) -> Vec<u8> {
    let body = LoginBody {
        u: String::from("filip@razek.org"),
        p: password,
    };
    let mut encoder = Encoder::new(Vec::new(), 0).unwrap();
    serde_json::to_writer(&mut encoder, &body).unwrap();
    encoder.finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_body_compression() {
        assert_eq!(build_request_body(String::from("password")), [40, 181, 47, 253, 0, 88, 49, 1, 0, 123, 34, 117, 34, 58, 34, 102, 105, 108, 105, 112, 64, 114, 97, 122, 101, 107, 46, 111, 114, 103, 34, 44, 34, 112, 34, 58, 34, 112, 97, 115, 115, 119, 111, 114, 100, 34, 125]);
        assert_eq!(build_request_body(String::from("c0Mpl1c4t3D_Pa$$w0Rd")), [40, 181, 47, 253, 0, 88, 145, 1, 0, 123, 34, 117, 34, 58, 34, 102, 105, 108, 105, 112, 64, 114, 97, 122, 101, 107, 46, 111, 114, 103, 34, 44, 34, 112, 34, 58, 34, 99, 48, 77, 112, 108, 49, 99, 52, 116, 51, 68, 95, 80, 97, 36, 36, 119, 48, 82, 100, 34, 125]);
        assert_eq!(build_request_body(String::from("letmein123")), [40, 181, 47, 253, 0, 88, 65, 1, 0, 123, 34, 117, 34, 58, 34, 102, 105, 108, 105, 112, 64, 114, 97, 122, 101, 107, 46, 111, 114, 103, 34, 44, 34, 112, 34, 58, 34, 108, 101, 116, 109, 101, 105, 110, 49, 50, 51, 34, 125]);
    }
}
