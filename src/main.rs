// https://github.com/iinc0gnit0/RBust
// You may copy this tool but please give credit :)
// Created by inc0gnit0 / skript0r
// v1.7
// 5/26/20

// Dependencies
use clap::{App, Arg}; // 2.33.1
use isahc::prelude::*; // 0.9.2
use rayon::prelude::*; // 1.3.0
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Duration;

// Main
fn main() -> std::io::Result<()> {
    banner();
    // Command line arguments
    let args = App::new("RBust")
        .version("v1.7")
        .author("inc0gnit0 <iinc0gnit0@pm.me> | skript0r <skript0r@protonmail.com>")
        .about("RBust is a blazing fast web directory bruteforce tool")
        .args_from_usage(
            "
            -u, --url=[TARGET_URL] 'Sets your target URL(required)'
            -w, --wordlist=[PATH_TO/_WORDLIST] 'Sets your wordlist file(required)'
            -t, --timeout=[SECONDS] 'Sets the timeout time in seconds Default(15)'
            -U, --user-agent=[USER_AGENT] 'Sets the user agent'",
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .help("Shows verbose output"),
        )
        .get_matches();

    let target_host = args.value_of("url").unwrap();
    let wordlist = args.value_of("wordlist").unwrap();
    let mut verbose = 0;
    let mut timeout = 15;
    let mut ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36"; // User agent
    match args.occurrences_of("v") {
        0 => verbose = 0,
        1 => verbose = 1,
        _ => println!(
            "\x1b[91mSomething went wrong!\nPlease make sure you typed everything right!\x1b[0m"
        ),
    };
    match args.occurrences_of("output") {
        0 => timeout = 15,
        1 => timeout = args.value_of("output").unwrap().parse::<u64>().unwrap(),
        _ => println!(
            "\x1b[91mSomething went wrong!\nPlease make sure you typed everything right!\x1b[0m"
        ),
    };
    match args.occurrences_of("user-agent") {
        0 => ua = ua,
        1 => ua = args.value_of("user-agent").unwrap(),
        _ => println!(
            "\x1b[91mSomething went wrong!\nPlease make sure you typed everything right!\x1b[0m"
        ),
    }
    // Read file
    let mut urls: Vec<String> = Vec::new();
    let fd = File::open(wordlist)?;
    for url in BufReader::new(fd).lines() {
        let url = url.unwrap();
        let url = url.trim().to_owned();
        urls.push(url);
    }
    urls.par_iter() // Making multithreaded requests
        .for_each(|url_path| 
            match probe(&target_host, &url_path, verbose, timeout, ua) {
                Ok(request) => request,
                Err(e) => println!("\x1b[31mSomething went wrong, please check if the URL is valid and try changing the timeout time\nError: {}\n\x1b[0m", e),
            });
    Ok(())
}

// Banner
fn banner() {
    println!(
        "\x1b[91m              https://github.com/iinc0gnit0/RBust

\x1b[93m   ▄████████ ▀█████████▄  ███    █▄     ▄████████     ███     
  ███    ███   ███    ███ ███    ███   ███    ███ ▀█████████▄ 
  ███    ███   ███    ███ ███    ███   ███    █▀     ▀███▀▀██ 
 ▄███▄▄▄▄██▀  ▄███▄▄▄██▀  ███    ███   ███            ███   ▀ 
▀▀███▀▀▀▀▀   ▀▀███▀▀▀██▄  ███    ███ ▀███████████     ███     
▀███████████   ███    ██▄ ███    ███          ███     ███     
  ███    ███   ███    ███ ███    ███    ▄█    ███     ███     
  ███    ███ ▄█████████▀  ████████▀   ▄████████▀     ▄████▀   \x1b[92mv1.7\x1b[93m
  ███    ███\x1b[92m      Created by: inc0gnit0 / skript0r
                                
            \x1b[91mUse command: ./rbust for help\x1b[0m\n"
    )
}

// Make requests
fn probe(
    host: &str,
    url: &str,
    verbose: i8,
    timeout: u64,
    ua: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let ua = format!("user-agent: {}", ua);
    let target = format!("{}/{}", &host, &url);
    let target = url_encode(&target);
    let response = Request::head(&target) // Make HEAD request
        .timeout(Duration::new(timeout, 0))
        .body(ua)?
        .send()?;
    // Intrepret reponse code
    if verbose == 0 {
        if response.status() == 404 {
            print!("");
        } else if response.status() == 200 {
            println!("\x1b[92m200 [+] {}\x1b[0m", target)
        } else if response.status() == 403 {
            println!("\x1b[93m403 [*] {}\x1b[0m", target)
        } else {
            println!("\x1b[93m{} [*] {}\x1b[0m", response.status(), target)
        }
    } else if verbose == 1 {
        if response.status() == 404 {
            println!("\x1b[91m404 [-] {}\x1b[0m", target);
        } else if response.status() == 200 {
            println!("\x1b[92m200 [+] {}\x1b[0m", target)
        } else if response.status() == 403 {
            println!("\x1b[93m403 [*] {}\x1b[0m", target)
        } else {
            println!("\x1b[93m{} [*] {}\x1b[0m", response.status(), target)
        }
    } else {
        println!("\x1b[91mSomething went wrong!\x1b[0m")
    }
    Ok(())
}

// Sanitize URL
fn url_encode(data: &str) -> String {
    fn str_to_ascii_num(char: &str) -> u8 {
        let chars: Vec<_> = char.bytes().map(|c| c as char).collect();
        return chars[0] as u8;
    }
    let unsafe_chars: Vec<&str> = vec![
        " ", "'", "\"", ">", "<", "#", "%", "{", "}", "|", "\\", "^", "~", "[", "]", "+",
    ];
    let unsafe_nums: Vec<u8> = unsafe_chars.iter().map(|c| str_to_ascii_num(c)).collect();
    let supplied_nums: Vec<u8> = data.bytes().map(|b| b as u8).collect();
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
    return buffer;
}
