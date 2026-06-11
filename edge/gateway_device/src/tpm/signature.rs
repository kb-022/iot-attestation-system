use tss_esapi::structures::Signature;
use tss_esapi::traits::Marshall;


//Transforms tss-esapi::Signature type
//into a standard rust byte vector
fn marshall_signature(signature: &Signature) -> Vec<u8> {
    let signature_vec = signature
        .marshall().expect("failed to marshall signature");
    signature_vec
}


//formats the TPM's signature correctly:
//This is due to first 5 bytes and middle bytes
//being TPM specific information
pub fn signature_as_32_bytes(signature: &Signature) -> Vec<u8> {
    let signature_vec = marshall_signature(&signature);
    let signature_r = signature_vec[6..38].to_vec();
    let signature_s = signature_vec[40..].to_vec();
    let mut signature_as_32_bytes = vec![];
    signature_as_32_bytes.extend_from_slice(&signature_r);
    signature_as_32_bytes.extend_from_slice(&signature_s);
    signature_as_32_bytes
}