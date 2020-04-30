// Created by inc0gnit0, <your_name_goes_here>, <your_name_goes_here>
// Version v0.3
// 4/29/20



// Dependencies
use std::env;
use reqwest;



// Main
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        request();
    }
}



// Make Request
fn request() -> Result<String, reqwest::error::Error>> {
    let mut response = reqwest::get("https://rust-lang.org")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    if response.status.trim() == "200" {
        println!!("Status {}", response.status());
    }

    Ok(());
}