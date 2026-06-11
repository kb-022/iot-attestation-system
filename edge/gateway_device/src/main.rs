mod tpm;
mod routes;
mod mqtt;

use reqwest::blocking::Client;
use crate::tpm::primary_key::{create_primary};
use crate::tpm::ec_enum::EcType;
use crate::routes::esdca_routes::ecdsa_enroll_setup;
use tss_esapi::{
    Context, TctiNameConf,
};
use crate::mqtt::async_subscribe::subscribe_example;
//use crate::tpm::ecdaa::commit_and_sign;
use crate::tpm::ecdsa::create_ecdsa_handle;

fn main(){
    //From: https://github.com/parallaxsecond/rust-tss-esapi/blob/main/tss-esapi/examples/hmac.rs
    // Create a new TPM context. This reads from the environment variable `TPM2TOOLS_TCTI` or `TCTI`
    //use `TCTI=device:/dev/tpmrm0` for the linux kernel
    let client = Client::new();
    let mut context = Context::new(
        TctiNameConf::from_environment_variable()
            .expect("Failed to get TCTI / TPM2TOOLS_TCTI from environment. Try `export TCTI=device:/dev/tpmrm0`"),
    )
        .expect("Failed to create Context");
    
    //create the primary key for the TPM to encrypt private keys
    //primary key construction is the same granted the parameters in
    let primary_key = create_primary(&mut context);
    
    
    //------------------------------------------------------------------------------------------------------------------------------
    //load the keys from file or create new files to read from
    let ecdsa_keys = EcType::EcDsa.load_or_create(&mut context, &primary_key);
    //enroll pi or verify enrollment
    //ecdsa key handle for signing operations
    let handle = create_ecdsa_handle(&mut context, primary_key.key_handle, &ecdsa_keys.public, &ecdsa_keys.enc_private);
    
    ecdsa_enroll_setup(&ecdsa_keys.public, &client);

    //Complete the ECDSA IoT flow
    subscribe_example(&client, &mut context, handle);
    //--------------------------------------------------------------------------------------------------------------------------------

/*   
    uncomment this for ECDAA, comment everything between the dotted lines above
    let ecdaa_keys = EcType::EcDaa.load_or_create(&mut context, &primary_key);
    let ecdaa_handle = context
        .execute_with_nullauth_session(|ctx| ctx
            .load(primary_key.key_handle,
                  ecdaa_keys.enc_private,  ecdaa_keys.public))
        .expect("EcDaa KeyHandle could not be created");
    
    commit_and_sign(ecdaa_handle,context);
    
 */
    
}
