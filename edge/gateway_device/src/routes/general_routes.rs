use reqwest::blocking::Client;
use serde::{Deserialize};

const VERIFY_SERVER: &str = "http://192.168.0.88:3000";

pub fn format_verify_route(path: &str) -> String {
    format!("{}{}", VERIFY_SERVER, path)
}



#[derive(Deserialize, Debug)]
pub struct Challenge{
    pub(crate) challenge: Vec<u8>
}

pub fn get_challenge(client: &Client) -> Result<Challenge, Box<dyn std::error::Error>> {
    let res = client.get(format_verify_route("/auth/challenge"))
        .send()?;

    if res.status().is_success(){
        let challenge:Challenge = res.json()?;
        Ok(challenge)
    }else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "")))
    }
}

