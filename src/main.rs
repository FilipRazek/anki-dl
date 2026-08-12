use std::println;
use std::env;
use send_login_request::send_login_request;
use dotenv::dotenv;

mod send_login_request;

fn main() {
    trpl::block_on(login());
}

async fn login() {
    let data = send_login_request(load_username(), load_password()).await.expect("Error loggin in to AnkiWeb");
    println!("{}", data.key);
}

fn load_username() -> String {
    dotenv().ok();
    env::var("ANKI_USERNAME").expect("Anki username not found in .env")
}

fn load_password() -> String {
    dotenv().ok();
    env::var("ANKI_PASSWORD").expect("Anki password not found in .env")
}
