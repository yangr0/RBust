// Dependencies
use reqwest;



// Main
fn main() -> Result<String, reqwest::Error>> {
    let mut response = reqwest::get("http://httpbin.org/get")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    println!("Status: {}", response.status());

    Ok(());
}