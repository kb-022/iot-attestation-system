use crate::routes::general_routes::format_verify_route;
use reqwest::blocking::Client;
use serde::Serialize;
use tss_esapi::structures::Public;

#[derive(Serialize)]
pub struct User{
    device_id: String,
    public_key: Vec<u8>,
    curve: String,
}

#[derive(Serialize)]
pub struct Response{
    device_id: String,
    data: Vec<u8>,
    signature: Vec<u8>,
}

pub fn ecdsa_enroll_setup(public_key: &Public, client: &Client){
    if let Public::Ecc {unique, parameters, ..} = public_key{
        let curve = format!("{:?}",parameters.ecc_curve());
        ecdsa_enroll_client(client, unique.x().to_vec(), unique.y().to_vec(), curve).expect("POST Failed");
    }else {
        println!("Key is not Ecc Public Key.");
    }
}

pub fn ecdsa_enroll_client(client: &Client, x: Vec<u8>, y: Vec<u8>, curve: String) -> Result<(), Box<dyn std::error::Error>>{
    let mut sec1_bytes = vec![0x04];
    sec1_bytes.extend_from_slice(&x);
    sec1_bytes.extend_from_slice(&y);
    let user = User{
        device_id: "kb-pi".to_string(),
        public_key: sec1_bytes,
        curve,
    };
    let res = client.post(format_verify_route("/enroll"))
        .json(&user)
        .send()?;

    if res.status().is_success(){
        println!("Enrolled successfully");
    }else if  res.status() == reqwest::StatusCode::CONFLICT{
        println!("Already enrolled");
    }
    else{
        println!("Server failed to enroll: {}", res.status());
    }
    Ok(())
}



pub fn post_ecdsa_response(client: &Client, device_id: String, data: Vec<u8>, signature: Vec<u8>) -> Result<(), Box<dyn std::error::Error>>{
    let response =  Response {
        device_id,
        data,
        signature
    };
    let res = client.post(format_verify_route("/auth/response"))
        .json(&response)
        .send()?;

    if res.status().is_success(){
        println!("Response successfully sent");
    }else {
        println!("Error sending response: {}", res.status());
    }
    Ok(())
}