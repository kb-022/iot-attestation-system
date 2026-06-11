use std::fs::File;
use std::io::{Read, Write};
use tss_esapi::attributes::ObjectAttributesBuilder;
use tss_esapi::Context;
use tss_esapi::interface_types::algorithm::{HashingAlgorithm, PublicAlgorithm};
use tss_esapi::interface_types::ecc::EccCurve::{BnP256, NistP256};
use tss_esapi::structures::{CreatePrimaryKeyResult, EcDaaScheme, EccPoint, EccScheme, HashScheme, KeyDerivationFunctionScheme, Private, Public, PublicBuilder, PublicEccParametersBuilder};
use tss_esapi::traits::{Marshall, UnMarshall};

pub enum EcType{
    EcDsa,
    EcDaa
}

pub struct EccKeyPair {
    pub(crate) public: Public,
    pub(crate) enc_private: Private,
}

impl EcType{
    //https://github.com/parallaxsecond/rust-tss-esapi/discussions/542
    pub fn load_or_create(&self,context: &mut Context, primary: &CreatePrimaryKeyResult) -> EccKeyPair{
        let (pub_file_name, priv_file_name) = match self {
            EcType::EcDsa => ("ecdsa_key.pub","ecdsa_key.priv"),
            EcType::EcDaa => ("ecdaa_key.pub","ecdaa_key.priv")
        };

        let pub_file = File::open(pub_file_name);
        let priv_file = File::open(priv_file_name);

        if let (Ok(mut pub_file), Ok(mut priv_file)) = (pub_file, priv_file) {
            let mut buf = vec![];
            let _ = pub_file.read_to_end(&mut buf);
            let public = Public::unmarshall(&buf).unwrap();
            buf.clear();
            let _ = priv_file.read_to_end(&mut buf);
            let enc_private = Private::try_from(buf.clone()).unwrap();

            return EccKeyPair {public, enc_private};
        }

        let key_pair = match self {
            EcType::EcDsa => self.create_keys(context, primary),
            EcType::EcDaa => self.create_keys(context, primary),
        };

        let public = key_pair.public;
        let enc_private = key_pair.enc_private;

        let mut pub_file = File::create(pub_file_name).unwrap();
        pub_file.write_all(&public.marshall().unwrap()).unwrap();

        let mut priv_file = File::create(priv_file_name).unwrap();
        priv_file.write_all(&enc_private).unwrap();

        EccKeyPair {public, enc_private}
    }

    fn create_keys(&self,context: &mut Context, primary: &CreatePrimaryKeyResult) -> EccKeyPair{
        let object_attributes = match self {
            //object_attributes from: https://github.com/parallaxsecond/rust-tss-esapi/blob/main/tss-esapi/examples/hmac.rs
            EcType::EcDsa => ObjectAttributesBuilder::new()
                .with_fixed_tpm(true)
                .with_fixed_parent(true)
                .with_st_clear(false)
                .with_sensitive_data_origin(true)
                .with_user_with_auth(true)
                // The key is used only for signing.
                .with_sign_encrypt(true)
                .with_decrypt(false)
                .with_restricted(false)
                .build()
                .expect("Failed to build object attributes for ecc key"),
            EcType::EcDaa => ObjectAttributesBuilder::new()
                .with_fixed_tpm(true)
                .with_fixed_parent(true)
                .with_st_clear(false)
                .with_sensitive_data_origin(true)
                .with_user_with_auth(true)
                // The key is used only for signing.
                .with_sign_encrypt(true)
                .with_decrypt(false)
                .with_restricted(true)
                .build()
                .expect("Failed to build object attributes for ecdaa key")
        };

        let ecc_params = match self{
            EcType::EcDsa => PublicEccParametersBuilder::new()
            .with_ecc_scheme(
                EccScheme::EcDsa(HashScheme::new(HashingAlgorithm::Sha256)),
            )
            .with_curve(NistP256)
            .with_is_signing_key(true)
            .with_is_decryption_key(false)
            .with_restricted(false)
            .with_key_derivation_function_scheme(KeyDerivationFunctionScheme::Null)
            .build()
            .unwrap() ,
            EcType::EcDaa => PublicEccParametersBuilder::new()
                .with_ecc_scheme(
                    EccScheme::EcDaa(EcDaaScheme::new(HashingAlgorithm::Sha256,0)),
                )
                .with_curve(BnP256)
                .with_is_signing_key(true)
                .with_is_decryption_key(false)
                .with_restricted(true)
                .with_key_derivation_function_scheme(KeyDerivationFunctionScheme::Null)
                .build()
                .unwrap()
        };

        let ecc_public_builder = PublicBuilder::new()
            .with_public_algorithm(PublicAlgorithm::Ecc)
            .with_name_hashing_algorithm(HashingAlgorithm::Sha256)
            .with_object_attributes(object_attributes)
            .with_ecc_parameters(ecc_params)
            .with_ecc_unique_identifier(EccPoint::default())
            .build()
            .unwrap();

        let (enc_private, public) = context
            .execute_with_nullauth_session(|ctx| {
                ctx.create(primary.key_handle, ecc_public_builder, None, None, None, None)
                    .map(|key| (key.out_private, key.out_public))
            })
            .unwrap();

        EccKeyPair {public, enc_private}
    }
}