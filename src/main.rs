// Created by inc0gnit0 / skript0r
// Version v0.0.6
// 5/5/20



// Dependencies
use reqwest; // 0.10.4
use std::fs::File;
use std::io::{BufRead, BufReader};



// Read from file
fn readfile() {
    let filepath = "default.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let url = "https://github.com/";
        let url = url.to_owned() + &line; // Add the URL and wordlist together
        println!("{}", url);
    }
}



// Make Request
#[tokio::main]
async fn request() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://github.com/")
        .await?;
    // Intrepreting the status code
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
    // Handling errors in request()
    let result = match request() {
        Ok(response) => response,
        Err(_) => "Request failed".to_owned()
    };

    println!("{}", result);
    readfile();
}