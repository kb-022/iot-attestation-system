/*use tss_esapi::Context;
use tss_esapi::handles::KeyHandle;
use tss_esapi::interface_types::algorithm::HashingAlgorithm;
use tss_esapi::interface_types::reserved_handles::Hierarchy;
use tss_esapi::structures::{EcDaaScheme, EccParameter, EccPoint, MaxBuffer, Signature, SignatureScheme};

pub struct InitialCommit{
    pub(crate) e: EccPoint,
    pub(crate) counter: u16
}

pub struct Credential{
    pub(crate) a: EccPoint,
    pub(crate) b: EccPoint,
    pub(crate) c: EccPoint,
    pub(crate) d: EccPoint,
    pub(crate) counter: u16
}
//Credentials
//taken directly from sage script - hardcoded
const A_X:[u8;32] = [48, 214, 224, 127, 188, 13, 121, 76, 133, 238, 193, 13, 75, 233, 255, 144, 217, 190, 87, 104, 51, 100, 7, 4, 189, 35, 34, 148, 220, 180, 170, 160];
const A_Y:[u8;32] =[89, 44, 44, 242, 43, 10, 0, 216, 252, 213, 27, 75, 85, 137, 191, 226, 188, 51, 205, 241, 139, 54, 108, 99, 33, 223, 52, 135, 139, 80, 82, 11];

const B_X:[u8;32] =[249, 215, 194, 128, 144, 51, 76, 79, 172, 6, 210, 155, 187, 88, 208, 30, 162, 26, 11, 151, 46, 99, 239, 128, 32, 36, 98, 197, 61, 147, 41, 146];
const B_Y:[u8;32] =[150, 140, 197, 153, 16, 81, 163, 40, 118, 165, 87, 39, 132, 28, 107, 221, 33, 235, 170, 246, 110, 177, 124, 231, 158, 10, 231, 195, 12, 241, 105, 84];
const C_X:[u8;32] =[98, 147, 178, 172, 111, 0, 255, 171, 147, 141, 33, 91, 134, 209, 241, 17, 130, 230, 100, 57, 240, 141, 249, 13, 104, 93, 228, 225, 86, 196, 70, 164];
const C_Y:[u8;32] =[93, 72, 232, 73, 147, 156, 50, 164, 194, 101, 222, 203, 240, 12, 44, 98, 41, 191, 83, 175, 181, 39, 189, 36, 51, 121, 231, 7, 143, 7, 245, 153];

//Run generate credential function - only hardcoded if verified in sage
const D_X:[u8;32] =[168, 170, 26, 109, 145, 24, 96, 205, 17, 69, 84, 11, 137, 217, 88, 62, 55, 135, 234, 88, 179, 1, 89, 0, 56, 61, 128, 191, 85, 82, 190, 212];
const D_Y:[u8;32] =[150, 105, 228, 142, 92, 85, 92, 74, 44, 47, 191, 206, 22, 197, 118, 7, 72, 26, 94, 250, 41, 121, 63, 239, 239, 186, 185, 225, 217, 4, 77, 141];

fn format_ecc_point(x: [u8;32], y: [u8;32]) -> EccPoint{
    EccPoint::new(EccParameter::try_from(x.to_vec()).unwrap(), EccParameter::try_from(y.to_vec()).unwrap())
}


// pub fn generate_host_side_credential(ecdaa_handle: KeyHandle,mut context: Context) -> Credential {
//
//     let a = format_ecc_point(A_X,A_Y);
//     let b = format_ecc_point(B_X,B_Y);
//     let c = format_ecc_point(C_X,C_Y);
//
//     let (_,_,d,counter) = context
//         .execute_with_nullauth_session(|ctx|
//             ctx.commit(ecdaa_handle,b.clone(),None,None))
//         .expect("Failed to commit");
//
//     Credential{
//         a,b,c,d,counter
//     }
// }


pub fn commit_and_sign(ecdaa_handle: KeyHandle,mut context: Context) -> Signature{
    let b = format_ecc_point(B_X,B_Y);
    let (k,l,e,counter) = context
        .execute_with_nullauth_session(|ctx|
            ctx.commit(ecdaa_handle,b,None,None))
        .expect("Failed to commit");
    println!("K: {:?}",k);
    println!("L: {:?}",l);
    println!("E: {:?}",e);
    println!("Counter: {:?}",counter);


    //https://docs.rs/tss-esapi/latest/tss_esapi/struct.Context.html#method.hash
    let input_data = MaxBuffer::try_from("There is no spoon".as_bytes().to_vec())
        .expect("Failed to create buffer for input data.");

    let (digest, ticket) = context
        .hash(
            input_data,
            HashingAlgorithm::Sha256,
            Hierarchy::Owner,
        )
        .expect("Call to hash failed.");

    println!("digest: {:?}",digest);

    let signature = context.execute_with_nullauth_session(|ctx|
        ctx.sign(
            ecdaa_handle,
            digest,
            SignatureScheme::EcDaa {scheme: EcDaaScheme::new(HashingAlgorithm::Sha256, counter) },
            ticket
        )
    ).expect("failed to create signature");

    println!("Signature: {:?}",signature);
    signature
}


*/