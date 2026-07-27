use std::println;
use std::env;
use decompress::decompress_result;
use login_request_body::{build_anki_sync_header, build_request_body};
use dotenv::dotenv;

mod decompress;
mod login_request_body;

fn main() {
    trpl::block_on(login());
}

async fn login() {
    /*
    POST https://sync.ankiweb.net/sync/hostKey
        Content-Type: application/octet-stream
        anki-sync: {"v":11,"k":"","c":"anki,2.x.x","s":"<any-session-id>"}
        <body = zstd( {"u":"<user>","p":"<pass>"} )>

     */
    dotenv().ok();
    let password = env::var("ANKI_PASSWORD").unwrap();

    let body = build_request_body(password);
    let client = reqwest::Client::new();

    let res = client.post("https://sync.ankiweb.net/sync/hostKey")
        .header("anki-sync", build_anki_sync_header())
        .header("Content-Type", "application/octet-stream")
        .body(body)
        .send()
        .await
        .unwrap();

    let bytes = res.bytes().await.unwrap();
    let data = decompress_result(&bytes[..]).unwrap();
    println!("{}", data.key);
}
