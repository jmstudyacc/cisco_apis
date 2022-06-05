extern crate core;

use hyper::body::HttpBody;
use hyper::header::SET_COOKIE;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    get().await?;

    // parse an http URI
    let mut uri: Uri = "https://sandbox-sdwan-1.cisco.com/j_security_check"
        .parse()
        .expect("Cannot parse URL");

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(Body::from("j_username=devnetuser&j_password=RG!_Yw919_83"))?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let mut resp = client.request(req).await?;

    println!("Body: {:?}", &resp.body());

    println!("\nGetting cookies...");
    // access the response headers and get the returned Cookies
    let cookies = &resp.headers().get(SET_COOKIE).unwrap();

    let auth_cookie = cookies.to_str().unwrap().split(";");

    println!("\n**** Printing JSESSION_ID... ****");
    let vec = auth_cookie.collect::<Vec<&str>>();
    println!("{:?}", &vec[0]);
    let jsessionid = vec[0];

    uri = "https://sandbox-sdwan-1.cisco.com/dataservice/client/token"
        .parse()
        .expect("Unable to parse");

    // TODO: Need to get the cookie programmatically and pass into header

    let auth = Request::builder()
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Cookie", jsessionid)
        .body(Body::empty())?;

    let mut auth_body = client.request(auth).await?;
    println!("\n**** Auth Response: {} ****", auth_body.status());

    let reader = auth_body
        .body_mut()
        .data()
        .await
        .expect("Unable to get XSRF Token ")
        .unwrap();

    let xsrf_token = String::from_utf8(reader.to_vec()).unwrap();

    //xsrf_token = xsrf_token.parse().unwrap();
    println!("{:?}", xsrf_token);

    println!("\nMake API call for Devices");

    let devices_uri: Uri = "https://sandbox-sdwan-1.cisco.com/dataservice/device".parse()?;

    // TODO: Need to insert JSESSIONID and XSRF Token programmatically

    let device = Request::builder()
        .uri(devices_uri)
        .header("Content-Type", "application/json")
        .header("Cookie", jsessionid)
        .header("X-XSRF-TOKEN", xsrf_token)
        .body(Body::empty())?;

    let mut device_list = client.request(device).await?;

    println!("**** Device GET Status: {} ****", device_list.status());

    while let Some(chunk) = device_list.body_mut().data().await {
        let vec = chunk;
        let res = String::from_utf8(vec.unwrap().to_vec()).unwrap();
        println!("{:?}", res);
        //stdout().write_all(&chunk?).await?;
    }

    Ok(())
}

async fn get() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // creating an HTTPS connector
    let https = HttpsConnector::new();

    // creating an Hyper client with builder pattern => including https connector
    let client = Client::builder().build::<_, Body>(https);

    // parse an http URI
    let uri = "https://sandbox-sdwan-1.cisco.com/j_security_check"
        .parse()
        .expect("Cannot parse URL");

    // Await the response
    let resp = client.get(uri).await.expect("Unable to get response.");

    println!("Got a response!");
    // print the HTTP status
    println!("Response: {}", resp.status());

    Ok(())
}
