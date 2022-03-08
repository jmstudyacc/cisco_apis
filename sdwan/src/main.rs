extern crate reqwest;

use sdwan::{vmanage_get, vmanage_login};
//use reqwest::StatusCode;

#[tokio::main]
async fn main() {
    vmanage_login(
        "https://sandbox-sdwan-1.cisco.com",
        "devnetuser",
        "RG!_Yw919_83",
    )
    .await;

    let res = vmanage_get(
        "https://sandbox-sdwan-1.cisco.com",
        "/dataservice/client/token",
    )
    .await;
    println!("{:?}", res.);
    // println!();
    // // bind a variable to hold the sdwan url
    // let endpoint_url = "https://sandbox-sdwan-1.cisco.com";
    // let login_action = "/j_security_check";
    // let login_url = endpoint_url.to_owned() + login_action;
    // let user = "devnetuser";
    // let password = "RG!_Yw919_83";

    // match res.error_for_status() {
    //     Ok(_res) => println!("A Great Success!"),
    //     Err(res) => println!("{:?}", res.status()),
    // };
    //println!("A Great success!");
    //println!("Response text: {:?}", res.text().await?)
    //}

    /*
    Using the client:
        1. Form a GET request
        2. SEND the request
        3. send() returns a FUTURE so we need to .await? it
    */
    //let result_debug = client.get(endpoint_url).send().await?;

    /*
    As above but with the following extra steps:
        4. text() returns the response in TEXT form
        5. a final await to return the future and the GET request in text form
    Once you do this the returned info is in text and you have lost a lot of manipulation
    */
    // let result_text = client.get(endpoint_url).send().await?.text().await?;
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    async fn compare_client_get() : test to compare the output of using
    the reqwest::client vs. not using the client but using the same
    builder patterns
    */
    #[tokio::test]
    async fn compare_client_get() -> Result<(), reqwest::Error> {
        let endpoint_url = "https://sandbox-sdwan-1.cisco.com";

        // if you don't plan to make multiple requests the below can be used
        let body = reqwest::get(endpoint_url).await?.text().await?;

        // creating a new reqwest client
        let client = reqwest::Client::new();
        let result = client.get(endpoint_url).send().await?.text().await?;

        // using a client or inline returns the same when using the same builder pattern
        Ok(assert_eq!(body, result))
    }
}
