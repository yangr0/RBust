// Dependencies
use reqwest;



// Main
fn main() -> Result<String, reqwest::Error>> {
    let mut response = reqwest::get("https://rust-lang.org")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    if response.status.trim() == "200" {
        println!!("Status {}", response.status());
    }

    Ok(());
}