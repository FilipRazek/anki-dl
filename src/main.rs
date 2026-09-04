use send_login_request::UserCredentials;
use send_login_request::send_login_request;

mod env_var;
mod send_login_request;

#[tokio::main]
async fn main() {
    login().await;
}

async fn login() {
    let login_data = send_login_request(UserCredentials {
        user: env_var::load_var("ANKI_USERNAME"),
        password: env_var::load_var("ANKI_PASSWORD"),
    })
    .await
    .expect("Error during login to AnkiWeb");
    println!("{}", login_data.key);
}
