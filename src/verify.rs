use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use jsonwebtoken::{Algorithm, crypto::aws_lc::DEFAULT_PROVIDER, DecodingKey};

use crate::error::Error;

/// Check token is using RS256 and does not change after it has been created
pub(crate) fn verify_rs256_is_error(token: &str, public_key_pem: &str) -> Result<bool, Error> {
    // A JWT token consists of 3 parts separated by dots(.)
    // header.payload.signature
    // header, payload and signature are Base64 encoding with URL-Safe alphabets without '=' padding
    // signature is an encryption of 'header.payload' text with a cipher algorithm defined by 'alg' field of token header
    let sig_msg = token.rsplitn(2, '.').collect::<Vec<&str>>();
    if sig_msg.len() == 2 {
        let payload_header = sig_msg[1].rsplitn(2, '.').collect::<Vec<&str>>();
        if payload_header.len() == 2 {
            let key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())?;
            let header_bytes = URL_SAFE_NO_PAD.decode(payload_header[1].as_bytes())?;
            let header_value = serde_json::from_slice::<serde_json::Value>(&header_bytes)?;
            if let Some(alg) = header_value["alg"].as_str() && alg == "RS256" {
                let verifying_provider = (DEFAULT_PROVIDER.verifier_factory)(&Algorithm::RS256, &key)?;
                let signature = URL_SAFE_NO_PAD.decode(sig_msg[0].as_bytes())?;
                Ok(verifying_provider.verify(sig_msg[1].as_bytes(), &signature).is_err())
            } else {
                Err(Error::new_custom("Token algorithm is not RS256"))
            }
        } else {
            Err(Error::new_custom("Malformed token"))
        }
    } else {
        Err(Error::new_custom("Malformed token"))
    }
}

// If you want to test, please replace 'public_key_pem_xxx' and 'token' with the valid values

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_verify_moph_id() {
//         let public_key_pem_block = r#"-----BEGIN PUBLIC KEY-----
// MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAmYODxZhjyq3EWxNlN59T
// …
// gY+hrnj/vMVUHy8caWXBlz5laz15SzX7GQisHrht8QF1yVxiuHiMOpLhpbJlLopP
// MSgSB17PdkYPmG9PrGBHNSkCAwEAAQ==
// -----END PUBLIC KEY-----
// "#;
//         let public_key_pem_inline = "-----BEGIN PUBLIC KEY-----MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAmYODxZhjyq3EWxNlN59T…gY+hrnj/vMVUHy8caWXBlz5laz15SzX7GQisHrht8QF1yVxiuHiMOpLhpbJlLopPMSgSB17PdkYPmG9PrGBHNSkCAwEAAQ==-----END PUBLIC KEY-----";
//         let public_key_pem_with_newline = "-----BEGIN PUBLIC KEY-----\nMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAmYODxZhjyq3EWxNlN59T\n…\ngY+hrnj/vMVUHy8caWXBlz5laz15SzX7GQisHrht8QF1yVxiuHiMOpLhpbJlLopP\nMSgSB17PdkYPmG9PrGBHNSkCAwEAAQ==\n-----END PUBLIC KEY-----\n";
//         let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIwMTk2YTU0M…CIsImlhbCI6Mi4zfX0.Ha9rFGhUfmR0D4hVAVIW1…fYCnD_8xjgbSFfIe93QAUc";
//         assert!(!verify_rs256_is_error(token, public_key_pem_block).unwrap());
//         assert!(!verify_rs256_is_error(token, public_key_pem_inline).unwrap());
//         assert!(!verify_rs256_is_error(token, public_key_pem_with_newline).unwrap());
//     }

//     #[test]
//     fn test_verify_provider_id() {
//         let public_key_pem_block = r#"-----BEGIN PUBLIC KEY-----
// MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEArhEKW+TAfB1xhM7RmaMp
// …
// b8kHQi0wc1XLZwXEV2DpSi9StUEP8uW6XsZFzdbLCu7Q3amc5guKk7lGNG5k0axA
// OeB7RNIm1YrnBCBi2KAks5UCAwEAAQ==
// -----END PUBLIC KEY-----
// "#;
//         let public_key_pem_inline = "-----BEGIN PUBLIC KEY-----MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEArhEKW+TAfB1xhM7RmaMp…b8kHQi0wc1XLZwXEV2DpSi9StUEP8uW6XsZFzdbLCu7Q3amc5guKk7lGNG5k0axAOeB7RNIm1YrnBCBi2KAks5UCAwEAAQ==-----END PUBLIC KEY-----";
//         let public_key_pem_with_newline = "\n        -----BEGIN PUBLIC KEY-----\n        MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEArhEKW+TAfB1xhM7RmaMp\n        …\n        b8kHQi0wc1XLZwXEV2DpSi9StUEP8uW6XsZFzdbLCu7Q3amc5guKk7lGNG5k0axA\n        OeB7RNIm1YrnBCBi2KAks5UCAwEAAQ==\n        -----END PUBLIC KEY-----\n        ";
//         let token = "eyJ0eXAiOiJKV1QiLC…zZjQyMTg3OGE5ODAyIn0.eyJhdWQiOiI1MDMiLCJqd…DMyODIiLCJzY29wZXMiOltdfQ.ayyu3c0S2oAauGa_GjhjwT…2aHYfzu6pP2muNphZcxq8FI";
//         assert!(!verify_rs256_is_error(token, public_key_pem_block).unwrap());
//         assert!(!verify_rs256_is_error(token, public_key_pem_inline).unwrap());
//         assert!(!verify_rs256_is_error(token, public_key_pem_with_newline).unwrap());
//     }
// }
