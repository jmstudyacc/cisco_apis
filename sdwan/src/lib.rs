//! in sdwan/src/lib.rs
use reqwest::{Client, Error, Response};
use std::collections::HashMap;

// as we are getting a Future for the http request we need to include #[tokio::main]
pub async fn vmanage_login(endpoint: &str, username: &str, password: &str) -> Result<(), Error> {
    let login_url = endpoint.to_string() + "/j_security_check";
    println!("{}", login_url);
    let login_data = HashMap::from([("j_username", username), ("j_password", password)]);
    let client = Client::new();
    let res = client.post(login_url).form(&login_data).send().await?;

    match res.error_for_status() {
        Ok(r) => println!("A great success! {:?}", r.status()),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
        }
    };

    Ok(())
}

pub async fn vmanage_get(endpoint: &str, mnt_point: &str) -> Response {
    let url = endpoint.to_string() + mnt_point;
    let client = reqwest::Client::new();
    let res = client.get(url).send().await;
    match res {
        Ok(r) => r,
        Err(e) => panic!(),
    }
}
