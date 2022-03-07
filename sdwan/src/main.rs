extern crate reqwest;

#[tokio::main]
// as we are getting a Future for the http request we need to include #[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // bind a variable to hold the sdwan url
    let endpoint_url = "https://sandbox-sdwan-1.cisco.com";

    // if you don't plan to make multiple requests the below can be used
    // let body = reqwest::get(endpoint_url).await?.text().await?;

    // creating a new reqwest client to use for making multiple requests
    let client = reqwest::Client::new();

    // using the client:
    //  1. Form a GET request
    //  2. SEND the request
    //  3. send() returns a FUTURE so we need to .await? it
    let result_debug = client.get(endpoint_url).send().await?;

    // like above but with the following extra steps:
    //  4. text() returns the response in TEXT form
    //  5. a final await to return the future and the GET request in text form
    // Once you do this the returned info is in text and you have lost a lot of manipulation
    // let result_text = client.get(endpoint_url).send().await?.text().await?;

    println!(
        "remote address: {:?}\nusername: {:?}\t|\tpassword: {:?}",
        result_debug
            .remote_addr()
            .expect("Unable to retrieve remote address."),
        result_debug.url().username(),
        result_debug.url().password()
    );
    println!("{:?}", result_debug);
    Ok(())
    //Ok(println!("{:?}", result_text))
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
