use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyStruct {
    city: String,
    name: String,
}

fn main() {
    let response = reqwest::blocking::get("https://api.mocki.io/v1/ce5f60e2").unwrap();
    let var: MyStruct = response.json().unwrap();
    println!("{:?}", var);
}
