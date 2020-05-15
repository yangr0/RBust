// https://github.com/iinc0gnit0/RBust
// You may copy this tool but please give credit :)
// Created by inc0gnit0 / skript0r
// v1.2
// 5/14/20



// Dependencies
use rayon::prelude::*; // 1.3.0
use isahc::prelude::*; // 0.9.2
use std::io::{BufReader, prelude::*};
use std::fs::File;
use clap::App;
use std::time::Duration;



// Main
fn main() -> std::io::Result<()> {
    banner();
    // Command line arguments
    let args = App::new("RBust")
        .version("v1.2")
        .author("inc0gnit0 <iinc0gnit0@pm.me> | skript0r <skript0r@protonmail.com>")
        .about("RBust is a blazing fast web directory bruteforce tool")
        .args_from_usage(
            "-u, --url=[TARGET_URL] 'Sets your target URL'")
        .args_from_usage(
            "-w, --wordlist=[PATH_TO_WORDLIST] 'Sets your wordlist file'")
        .get_matches();

    let target_host = args.value_of("url").unwrap();
    let wordlist = args.value_of("wordlist").unwrap(); 
    // Read file
    let mut urls:Vec<String> = Vec::new();
    let fd = File::open(wordlist)?;
    for url in BufReader::new(fd).lines() {
    let url = url.unwrap();
    let url = url.trim().to_owned();
    urls.push(url);
    }
    urls.par_iter().for_each(|url_path| probe(&target_host, &url_path).unwrap());
    Ok(())
}



// Banner
fn banner() {
    println!("\x1b[91m              https://github.com/iinc0gnit0/RBust

\x1b[93m   ▄████████ ▀█████████▄  ███    █▄     ▄████████     ███     
  ███    ███   ███    ███ ███    ███   ███    ███ ▀█████████▄ 
  ███    ███   ███    ███ ███    ███   ███    █▀     ▀███▀▀██ 
 ▄███▄▄▄▄██▀  ▄███▄▄▄██▀  ███    ███   ███            ███   ▀ 
▀▀███▀▀▀▀▀   ▀▀███▀▀▀██▄  ███    ███ ▀███████████     ███     
▀███████████   ███    ██▄ ███    ███          ███     ███     
  ███    ███   ███    ███ ███    ███    ▄█    ███     ███     
  ███    ███ ▄█████████▀  ████████▀   ▄████████▀     ▄████▀   \x1b[92mv1.2\x1b[93m
  ███    ███\x1b[92m      Created by: inc0gnit0 / skript0r\n")
}



// Make requests
fn probe(host:&str, url:&str) -> Result<(), Box<dyn std::error::Error>>{
    let target = format!("{}/{}", &host, &url);
    let target = url_encode(&target);
    let response = Request::head(&target) // Make HEAD request
        .timeout(Duration::new(1,0))
        .body("")?
        .send()?;
    // Intrepret reponse code
    if response.status() == 404 {
        print!("");
    } else if response.status() == 200 {
        println!("\x1b[92m200 [+] {}", target)
    } else if response.status() == 403 {
        println!("\x1b[93m403 [*] {}", target)
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