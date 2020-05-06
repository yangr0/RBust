// Created by inc0gnit0 / skript0r
// Version v0.0.7
// 5/6/20



// Dependencies
use reqwest; // 0.10.4
use std::fs::File;
use std::io::{BufRead, BufReader};



// Colors
const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";
const RESET: &str = "\x1b[0m";



// Read from file
fn readfile() {
    let filepath = "default.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let url = "https://github.com/";
        let full_url = url.to_owned() + &line; // Add the URL and wordlist together
        // Output
        let result = match request(full_url) {
            Ok(response) => response + url + &line,
            Err(_) => "\x1b[91m[!]Request Failed, please check if you have internet connection".to_owned(),
        };
        println!("{}", result);
    }
}



// Make Request
#[tokio::main]
async fn request(url: String) -> Result<String, reqwest::Error> {
    let response = reqwest::get(&url)
        .await?;
    // Intrepreting the status code
    let result =
        if response.status() == 404 {
            "\x1b[91m[-]".to_owned()
        } else {
            "\x1b[92m[+]".to_owned()
        };
    Ok(result)

}



// Main
fn main() {
    // Handling errors in request()
    readfile();
}
