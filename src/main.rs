use std::println;
use std::env;
use decompress::decompress_result;
use send_login_request::send_login_request;
use dotenv::dotenv;

mod decompress;
mod send_login_request;

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
    let res = send_login_request(load_password()).await.unwrap();
    let bytes = res.bytes().await.unwrap();
    let data = decompress_result(&bytes[..]).unwrap();
    println!("{}", data.key);
}

fn load_password() -> String {
    dotenv().ok();
    env::var("ANKI_PASSWORD").unwrap()
}