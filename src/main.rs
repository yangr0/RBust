// Created by inc0gnit0, <your_name_goes_here>, <your_name_goes_here>
// Version v0.3
// 4/29/20



// Dependencies
use std::env;
use reqwest;



// Main
fn main() {
    match request() {
        Ok(res) => res,
        Err(_) => "Error".to_owned()
    };
}



// Make Request
fn request() {
    let response = reqwest::get("https://rustlang.org");
    let result = if response.status() == 404 {
        "Page does not exist".to_owned()
    } else {
        "Page exist".to_owned()
    };

    Ok(result);
}