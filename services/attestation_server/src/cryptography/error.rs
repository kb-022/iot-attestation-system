use std::{error, fmt};
use p256::elliptic_curve;

#[derive(Debug)]
pub enum CryptoError{
    Curve(elliptic_curve::Error),
    Database(sqlx::Error),
    EcDsa(p256::ecdsa::Error),
}

impl fmt::Display for CryptoError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            CryptoError::Curve(ref e) => write!(f, "Curve error: {}", e),
            CryptoError::Database(ref e) => write!(f, "Database error: {}", e),
            CryptoError::EcDsa(ref e) => write!(f, "EcDsa error: {}", e),
        }
    }
}

impl error::Error for CryptoError{
    fn source(&self) -> Option<&(dyn error::Error + 'static)>{
        match *self{
            CryptoError::Curve(ref e) => Some(e),
            CryptoError::Database(ref e) => Some(e),
            CryptoError::EcDsa(ref e) => Some(e),
        }
    }
}