// Created by inc0gnit0 / skript0r
// Version v0.3
// 4/29/20



// Dependencies
use reqwest;



// Make Request
#[tokio::main]
async fn request() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://rustlang.org")
        .await?;
    let result =
        if response.status() == 404 {
            "Page Not Found".to_owned()
        } else {
            "Page Found".to_owned()
        };
    Ok(result)
}



// Main
fn main() {
    let result = match request() {
        Ok(response) => response,
        Err(_) => "Request failed".to_owned()
    };

    println!("{}", result);
}