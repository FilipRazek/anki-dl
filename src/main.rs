use send_login_request::send_login_request;
use std::println;

mod send_login_request;
mod env_var;

#[tokio::main]
async fn main() {
    login().await;
}

async fn login() {
    let login_data = send_login_request(env_var::load_var("ANKI_USERNAME"), env_var::load_var("ANKI_PASSWORD"))
        .await
        .expect("Error during login to AnkiWeb");
    println!("{}", login_data.key);
}

