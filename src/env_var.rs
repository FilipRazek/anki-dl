use dotenvy::dotenv;
use std::env;

pub fn load_username() -> String {
    dotenv().ok();
    env::var("ANKI_USERNAME").expect("Anki username not found in .env")
}

pub fn load_password() -> String {
    dotenv().ok();
    env::var("ANKI_PASSWORD").expect("Anki password not found in .env")
}
