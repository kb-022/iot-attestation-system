use tss_esapi::attributes::ObjectAttributesBuilder;
use tss_esapi::interface_types::algorithm::{HashingAlgorithm, PublicAlgorithm};
use tss_esapi::structures::{CreatePrimaryKeyResult, Digest, PublicBuilder, SymmetricCipherParameters, SymmetricDefinitionObject};
use tss_esapi::Context;
use tss_esapi::interface_types::reserved_handles::Hierarchy;

//From: https://github.com/parallaxsecond/rust-tss-esapi/blob/main/tss-esapi/examples/rsa_oaep.rs
// Create the primary key. A primary key is the "root" of a collection of objects.
// These other objects are encrypted by the primary key allowing them to persist
// over a reboot and reloads.
//
// A primary key is derived from a seed, and provided that the same inputs are given
// the same primary key will be derived in the tpm. This means that you do not need
// to store or save the details of this key - only the parameters of how it was created.
pub fn create_primary(context: &mut Context) -> CreatePrimaryKeyResult {
    let object_attributes = ObjectAttributesBuilder::new()
        // Indicate the key can only exist within this tpm and can not be exported.
        .with_fixed_tpm(true)
        // The primary key and it's descendent keys can't be moved to other primary keys
        .with_fixed_parent(true)
        // The primary key will persist over suspend and resume of the system.
        .with_st_clear(false)
        // The primary key was generated entirely inside the TPM - only this TPM knows it's content.
        .with_sensitive_data_origin(true)
        // This key requires "authentication" to the TPM to access - this can be
        // an HMAC or password session. HMAC sessions are used by default with
        // the "execute_with_nullauth_session" function.
        .with_user_with_auth(true)
        // This key has the ability to decrypt
        .with_decrypt(true)
        // This key may only be used to encrypt or sign objects that are within
        // the TPM - it can not encrypt or sign external data.
        .with_restricted(true)
        .build()
        .expect("Failed to build object attributes for primary key");

    let primary_pub = PublicBuilder::new()
        .with_public_algorithm(PublicAlgorithm::SymCipher)
        .with_name_hashing_algorithm(HashingAlgorithm::Sha256)
        .with_object_attributes(object_attributes)
        .with_symmetric_cipher_parameters(SymmetricCipherParameters::new(
            SymmetricDefinitionObject::AES_128_CFB,
        ))
        .with_symmetric_cipher_unique_identifier(Digest::default())
        .build()
        .unwrap();

    context
        .execute_with_nullauth_session(|ctx| {
            // Create the key under the "owner" hierarchy. Other hierarchies are platform
            // which is for boot services, null which is ephemeral and resets after a reboot,
            // and endorsement which allows key certification by the TPM manufacturer.
            ctx.create_primary(Hierarchy::Owner, primary_pub, None, None, None, None)
        })
        .unwrap()
}