//! # sjcl
//! Simple decrypt-only SJCL library.
//!
//! Only supports AES-CCM so far, but OCB2 is deprecated AFAIK.
//! To use you only need the result of a SJCL encrypted secret and the
//! passphrase:
//!
//! ## Usage
//! Packagename for your `Cargo.toml`:
//! ```toml
//! sjcl = "0.0.1"
//! ```
//!
//! Decrypt a file loaded into a string:
//! ```rust
//! use sjcl::decrypt_raw;
//! 
//! let data = "{\"iv\":\"nJu7KZF2eEqMv403U2oc3w==\", \"v\":1, \"iter\":10000, \"ks\":256, \"ts\":64, \"mode\":\"ccm\", \"adata\":\"\", \"cipher\":\"aes\", \"salt\":\"mMmxX6SipEM=\", \"ct\":\"VwnKwpW1ah5HmdvwuFBthx0=\"}".to_string();
//! let password_phrase = "abcdefghi".to_string();
//! let plaintext = decrypt_raw(data, password_phrase)?;
//! ```
//!
//! This will give you the plaintext `test\ntest`.
extern crate base64;

use ccm::{Ccm, consts::{U8, U13}};
use ccm::aead::{Aead, NewAead, generic_array::GenericArray};
use aes::Aes256;
use password_hash::{SaltString, PasswordHasher};
use pbkdf2::{Pbkdf2, Params};
use serde::Deserialize;
use serde_json;

use snafu::Snafu;
#[derive(Debug, Snafu)]
pub enum SjclError {
    #[snafu(display("Failed to decrypt chunk: {}", message))]
    DecryptionError { message: String },
    #[snafu(display("Method is not yet implemented"))]
    NotImplementedError,
}

#[derive(Debug, Deserialize)]
pub struct SjclBlockJson {
    iv: String,
    v: u32,
    iter: u32,
    ks: usize,
    ts: usize,
    mode: String,
    adata: String,
    cipher: String,
    salt: String,
    ct: String,
}

type AesCcm = Ccm<Aes256, U8, U13>;

/// Decrypts a chunk of SJCL encrypted JSON with a given passphrase.
pub fn decrypt_raw(chunk: String, key: String) -> Result<String, SjclError> {
    match serde_json::from_str(&chunk) {
        Ok(chunk) => decrypt(chunk, key),
        Err(_) => return Err(SjclError::DecryptionError {
            message: "Failed to parse JSON".to_string(),
        }),
    }
}

/// Utility function to trim the initialization vector to the proper size of
/// the nonce.
/// (See: [SJCL/core.ccm.js](https://github.com/bitwiseshiftleft/sjcl/blob/master/core/ccm.js#L61))
fn truncate_iv(mut iv: Vec<u8>, output_size: usize, tag_size: usize) -> Vec<u8> {
    let iv_size = iv.len();
    let output_size = (output_size - tag_size) / 8;

    let mut l = 2;
    while l < 4 && ((output_size >> (8 * l)) > 0) {
        l += 1
    }
    if iv_size <= 15 && l < 15 - iv_size {
        l = 15 - iv_size
    }

    let _ = iv.split_off(15 - l);
    iv
}

/// Decrypts a chunk of SJCL encrypted JSON with a given passphrase.
pub fn decrypt(mut chunk: SjclBlockJson, key: String) -> Result<String, SjclError> {
    match chunk.cipher.as_str() {
        "aes" => {
            match chunk.mode.as_str() {
                "ccm" => {
                    if chunk.v != 1 {
                        return Err(SjclError::DecryptionError {
                            message: "Only version 1 is currently supported".to_string(),
                        });
                    }
                    if chunk.adata.len() > 0 {
                        return Err(SjclError::DecryptionError {
                            message: "Expected empty additional data".to_string(),
                        });
                    }

                    let salt_str = match base64::decode(chunk.salt) {
                        Ok(v) => SaltString::b64_encode(&v),
                        Err(_) => return Err(SjclError::DecryptionError {
                            message: "Failed to base64 decode salt".to_string(),
                        }),
                    };
                    let salt = salt_str.unwrap();
                    let password_hash = Pbkdf2.hash_password(
                        key.as_bytes(),
                        None, 
                        None,
                        Params{rounds: chunk.iter, output_length: chunk.ks / 8},
                        salt.as_salt(),
                    );
                    let password_hash = match password_hash {
                        Ok(pwh) => pwh,
                        Err(_) => return Err(SjclError::DecryptionError {
                            message: "Failed to generate password hash".to_string(),
                        }),
                    };
                    let password_hash = password_hash.hash.unwrap();

                    // Fix missing padding
                    for _ in 0..(chunk.iv.len() % 4) {
                        chunk.iv.push('=');
                    }
                    for _ in 0..(chunk.ct.len() % 4) {
                        chunk.ct.push('=');
                    }
                    let iv = match base64::decode(chunk.iv) {
                        Ok(v) => v,
                        Err(_) => return Err(SjclError::DecryptionError {
                            message: "Failed to decode IV".to_string(),
                        }),
                    };
                    let ct = match base64::decode(chunk.ct) {
                        Ok(v) => v,
                        Err(_) => return Err(SjclError::DecryptionError {
                            message: "Failed to decode ct".to_string(),
                        }),
                    };
                    let iv = truncate_iv(iv, ct.len() * 8, chunk.ts);
                    let nonce  = GenericArray::from_slice(iv.as_slice());
                    let key = GenericArray::from_slice(password_hash.as_bytes());
                    let cipher = AesCcm::new(key);
                    let plaintext = match cipher.decrypt(nonce, ct.as_ref()) {
                        Ok(pt) => pt,
                        Err(_) => {
                            return Err(SjclError::DecryptionError {
                                message: "Failed to decrypt ciphertext".to_string(),
                            });
                        },
                    };
                    Ok(String::from_utf8(plaintext).unwrap())
                 },
                "ocb2" => {
                    Err(SjclError::NotImplementedError)
                },
                _ => Err(SjclError::NotImplementedError)
            }
        },
        _ => Err(SjclError::NotImplementedError)
    }
}

/// https://bitwiseshiftleft.github.io/sjcl/demo/
#[cfg(test)]
mod tests {
    use crate::{decrypt, decrypt_raw, SjclBlockJson};

    #[test]
    fn test_end_to_end() {
        let data = "{\"iv\":\"nJu7KZF2eEqMv403U2oc3w==\", \"v\":1, \"iter\":10000, \"ks\":256, \"ts\":64, \"mode\":\"ccm\", \"adata\":\"\", \"cipher\":\"aes\", \"salt\":\"mMmxX6SipEM=\", \"ct\":\"VwnKwpW1ah5HmdvwuFBthx0=\"}".to_string();
        let password_phrase = "abcdefghi".to_string();

        let plaintext = "test\ntest".to_string();

        assert_eq!(decrypt_raw(data, password_phrase).unwrap(), plaintext);
    }

    #[test]
    fn test_with_struct() {
        let data = SjclBlockJson {
            iv: "nJu7KZF2eEqMv403U2oc3w".to_string(),
            v: 1,
            iter: 10000,
            ks: 256,
            ts: 64,
            mode: "ccm".to_string(),
            adata: "".to_string(),
            cipher: "aes".to_string(),
            salt: "mMmxX6SipEM".to_string(),
            ct: "VwnKwpW1ah5HmdvwuFBthx0=".to_string(),
        };
        let password_phrase = "abcdefghi".to_string();

        let plaintext = "test\ntest".to_string();

        assert_eq!(decrypt(data, password_phrase).unwrap(), plaintext);
    }
}
