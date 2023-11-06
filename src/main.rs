use tokio;
use reqwest::blocking::Client; 



fn main() -> Result<(), reqwest::Error> {

    let client = Client::new();

    let resp = client.get("http://httpbin.org/").send()?;

    if resp.status().is_success() {

        println!("Request was successful!");
    } else {
        eprintln!("Request failed with status code: {}", resp.status());
    }

    Ok(())

    
}









