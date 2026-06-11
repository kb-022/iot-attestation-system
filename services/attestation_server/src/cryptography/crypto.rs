use crate::cryptography::error::CryptoError;
use crate::routes::auth_routes::Response;
use crate::user::user::User;
use p256::ecdsa::signature::hazmat::PrehashVerifier;
use p256::ecdsa::{Signature, VerifyingKey};
use p256::elliptic_curve;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Challenge{
    challenge: Vec<u8>
}

//Constructs tss-esapi::VerifyingKey from the known public key in
//database. This is used to verify signed data.
pub async fn construct_verifying_key(device_id: String, pool: &PgPool) -> Result<VerifyingKey,CryptoError>{
    let user_query =  User::get_by_id(pool,&device_id).await;
    match user_query{
        Ok(user)=>{
            if user.curve == "NistP256"{
                let sec1_bytes = user.public_key;
                let verifying_key = VerifyingKey::from_sec1_bytes(&sec1_bytes).expect("Invalid SEC1 bytes");
                Ok(verifying_key)
            }else{
                Err(CryptoError::Curve(elliptic_curve::Error))
            }
        },
        Err(e)=>{
            Err(CryptoError::Database(e))
        }
    }
}

//generates random number from system
//as system generates, assumed to be cryptographically secure
pub fn generate_challenge() -> Challenge{
    let mut buf = [0u8; 24];
    getrandom::fill(&mut buf).expect("Failed to generate challenge");
    Challenge{
        challenge: buf.to_vec()
    }
}

//Verify that the challenge the client received is signed by
//users known public key
pub async fn verify_response(device_id: String, response: Response, pool: &PgPool) -> Result<bool,CryptoError>{
    let user_verifying_key = construct_verifying_key(device_id.clone(), &pool).await.expect("Failed to generate user public key");
    let data = response.data;
    let signature = response.signature;
    let crypto_signature = Signature::from_slice(&signature)
        .expect("Failed to parse signature");
    let result = user_verifying_key.verify_prehash(&data, &crypto_signature);
    match result {
        Ok(()) => {
            Ok(true)
        }
        Err(e)=>{
            Err(CryptoError::EcDsa(e))
        }
    }
}




