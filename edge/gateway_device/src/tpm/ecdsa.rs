use crate::routes::esdca_routes::post_ecdsa_response;
use crate::routes::general_routes::get_challenge;
use crate::tpm::signature::signature_as_32_bytes;
use reqwest::blocking::Client;
use tss_esapi::handles::KeyHandle;
use tss_esapi::interface_types::algorithm::HashingAlgorithm;
use tss_esapi::structures::{Digest, HashScheme, HashcheckTicket, MaxBuffer, Private, Public, Signature, SignatureScheme};
use tss_esapi::Context;
use tss_esapi::interface_types::reserved_handles::Hierarchy;

struct EcDsaData {
    raw: Vec<u8>,
    digest: Digest,
}

//load ecdsa key pair into TPM
pub fn create_ecdsa_handle( context: &mut Context, primary_handle: KeyHandle ,public_key: &Public,private_key: &Private) -> KeyHandle{
    let private_key_clone = private_key.clone();
    let public_key_clone = public_key.clone();
    context
        .execute_with_nullauth_session(|ctx| ctx
            .load(primary_handle,
                  private_key_clone, public_key_clone))
        .expect("EcDsa KeyHandle could not be created")
}

//Null Hashcheck Ticket is required when ecdsa key
// is left unrestricted
fn create_null_hashcheck_ticket(context: &mut Context) -> HashcheckTicket{
    let (_, ticket) = context
        .hash(MaxBuffer::default(),HashingAlgorithm::Sha256,Hierarchy::Null)
        .expect("Failed to create ticket");
    ticket
}

//Format the challenge from the server specifically for this
//ecdsa flow
fn handle_data_for_ecdsa(challenge: Vec<u8>, payload: Vec<u8>) -> EcDsaData {
    let mut data: Vec<u8> = vec![];
    data.extend_from_slice(&challenge);
    data.extend_from_slice(&payload);

    let data_for_post = data.clone();

    let data_digest = Digest::try_from(data)
        .expect("Failed to convert Vec<u8> to Digest");
    EcDsaData {
        raw: data_for_post,
        digest: data_digest,
    }
}

//Sign the server generated challenge with the ecdsa keys
//present in the TPM
fn ecdsa_sign_challenge(context: &mut Context, ecdsa_handle: KeyHandle, digest: Digest, hashcheck_ticket: HashcheckTicket) -> Signature{
    let signature = context
        .execute_with_nullauth_session(|ctx|{
            ctx.sign(ecdsa_handle, digest,
                     SignatureScheme::EcDsa {
                         scheme: HashScheme::new(HashingAlgorithm::Sha256)},
                     hashcheck_ticket,
            )
        }).expect("Failed to sign challenge");
    signature
}

//Handle the complete flow
//sign -> post
pub fn ecdsa(client: &Client, context: &mut Context,ecdsa_handle: KeyHandle,payload: Vec<u8> ) {
    let ticket = create_null_hashcheck_ticket(context);
    //GET challenge from server
    let challenge = get_challenge(client)
        .expect("Failed to get challenge from server").challenge;
    //Helper function to format data (challenge || data)
    let data = handle_data_for_ecdsa(challenge,payload);
    //TPM sign operation
    let signature =
        ecdsa_sign_challenge(context, ecdsa_handle, data.digest,ticket);
    //POST to server for verification
    post_ecdsa_response(&client, "kb-pi".to_string(),
                        data.raw,
                        signature_as_32_bytes(&signature))
        .expect("Failed to call response post");
}