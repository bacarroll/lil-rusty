use backoff::{Error, ExponentialBackoff, Operation};
use std::time::Duration;

use std::io::Read;

fn fetch_url(url: &str) -> Result<String, Error<reqwest::Error>> {
    let mut op = || {
        println!("Fetching {}", url);
        let mut resp = reqwest::blocking::get(url)?;

        let mut content = String::new();
        let _ = resp.read_to_string(&mut content);
        Ok(content)
    };

    let mut backoff = ExponentialBackoff::default();
    backoff.max_elapsed_time = Some(Duration::from_secs(20));
    op.retry(&mut backoff)
}

fn main() {
    match fetch_url("https://www.rust-bfldsjk.org") {
        Ok(_) => println!("Sucessfully fetched"),
        Err(err) => panic!("Failed to fetch: {}", err),
    }
}