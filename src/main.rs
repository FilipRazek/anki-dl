use dotenvy::dotenv;
use send_login_request::send_login_request;
use std::env;
use std::println;

mod send_login_request;

#[tokio::main]
async fn main() {
    login().await;
}

async fn login() {
    let login_data = send_login_request(load_username(), load_password())
        .await
        .expect("Error during login to AnkiWeb");
    println!("{}", login_data.key);
}

fn load_username() -> String {
    dotenv().ok();
    env::var("ANKI_USERNAME").expect("Anki username not found in .env")
}

fn load_password() -> String {
    dotenv().ok();
    env::var("ANKI_PASSWORD").expect("Anki password not found in .env")
}
