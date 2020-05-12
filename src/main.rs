// Created by inc0gnit0 / skript0r
// v0.0.9 
// 5/10/20



// Dependencies
use rayon::prelude::*; // 1.3.0
use isahc::prelude::*; // 0.9.2
use std::io::{BufReader, prelude::*};
use std::fs::File;
use std::env::args;
use std::time::Duration;
use std::process::exit;



// Main
fn main() -> std::io::Result<()> {
    let target_host = check_args();
    let mut urls:Vec<String> = Vec::new();
    let fd = File::open("default.txt")?;
    for url in BufReader::new(fd).lines() {
    let url = url.unwrap();
    let url = url.trim().to_owned();
    urls.push(url);
    }
    urls.par_iter().for_each(|url_path| probe(&target_host, &url_path).unwrap());
    Ok(())
}



// Check arguments
fn check_args() -> String {
    let args = args().map(|a| a.to_owned()).collect::<Vec<String>>();
    if args.len() != 2 {
        show_usage();
    }
    return args[1].to_owned()
}



// Usages message
fn show_usage() {
    println!("Usage: ./rbust <https://example.com>");
    exit(1);
}



// Make requests
fn probe(host:&str, url:&str) -> Result<(), Box<dyn std::error::Error>>{
    let target = format!("{}/{}", &host, &url);
    let target = url_encode(&target);
    let response = Request::head(&target)
        .timeout(Duration::new(1,0))
        .body("")?
        .send()?;
    if response.status() == 404 {
        println!("\x1b[91m404 [-] {}", target);
    } else if response.status() == 200 {
        println!("\x1b[92m200 [+] {}", target)
    } else {
        println!("\x1b[93m{} [*] {}", response.status(), target)
    }
    Ok(())
}

// Sanitize URL
fn url_encode(data: &str) -> String {
    fn str_to_ascii_num(char: &str) -> u8 {
        let chars: Vec<_> = char.bytes().map(|c| c as char).collect();
        return chars[0] as u8
    }
    let unsafe_chars:   Vec<&str>  = vec![" ", "'", "\"", ">", "<", "#", "%", "{", "}", "|", "\\", "^", "~", "[", "]", "+"];
    let unsafe_nums:    Vec<u8>    = unsafe_chars.iter().map(|c| str_to_ascii_num(c)).collect();
    let supplied_nums:  Vec<u8>    = data.bytes().map(|b| b as u8).collect();
    let mut buffer = String::new();
    for num in supplied_nums {
        if unsafe_nums.contains(&num) {
        let sanitized = format!("%{:x?}", num).to_uppercase();
        buffer.push_str(&sanitized);
        } else {
        let sanitized = format!("{}", num as char);
        buffer.push_str(&sanitized);
        }
    }
    return buffer
}