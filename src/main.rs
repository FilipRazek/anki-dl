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

    let result = get_body(password);

    let client = reqwest::Client::new();

    let res = client.post("https://sync.ankiweb.net/sync/hostKey")
        .header("anki-sync", serde_json::to_string(&login_header).unwrap())
        .header("Content-Type", "application/octet-stream")
        .body(result)
        .send()
        .await
        .unwrap();

    let bytes = res.bytes().await.unwrap();
    let decoder = Decoder::new(&bytes[..]).unwrap();
    let data: LoginResult = serde_json::from_reader(decoder).unwrap();
    println!("{}", data.key);
}

fn get_body(password: String) -> Vec<u8> {
    let body = LoginBody {
        u: String::from("filip@razek.org"),
        p: password,
    };
    let mut encoder = Encoder::new(Vec::new(), 0).unwrap();
    serde_json::to_writer(&mut encoder, &body).unwrap();
    encoder.finish().unwrap()
}
