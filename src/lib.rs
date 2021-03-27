extern crate base64;

use ccm::{Ccm, consts::{U8}};
use ccm::aead::{Aead, NewAead, generic_array::GenericArray};
use aes::{Aes128, Aes256};
use serde::Deserialize;

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
    iv: String,  // Nonce
    v: i32,
    iter: u32,
    ks: u32,
    ts: u32,
    mode: String,
    adata: String,
    cipher: String,
    salt: String,
    ct: String,
}

type AesCcm128 = Ccm<Aes128, U8, U8>;
type AesCcm256 = Ccm<Aes256, U8, U8>;

pub fn decrypt(chunk: SjclBlockJson, key: String) -> Result<String, SjclError> {
    match chunk.cipher.as_str() {
        "aes" => {
            match chunk.mode.as_str() {
                "ccm" => {
                    let nonce = match base64::decode(chunk.iv) {
                        Ok(v) => v,
                        Err(_) => return Err(SjclError::DecryptionError {
                            message: "Failed to read IV".to_string(),
                        }),
                    };
                    let nonce = GenericArray::from_slice(&nonce);
                    let key = GenericArray::from_slice(key.as_bytes());
                    if chunk.ks == 256 {
                        let cipher = AesCcm256::new(key);

                        let plaintext = cipher.decrypt(nonce, chunk.ct.as_ref());
                    } else if chunk.ks == 128 {
                        let cipher = AesCcm256::new(key);

                        let plaintext = cipher.decrypt(nonce, chunk.ct.as_ref());
                    }

                    Err(SjclError::NotImplementedError)
                 },
                "ocb2" => {
                    if chunk.ks == 128 && chunk.iter == 1000 && chunk.ts == 64 {
                        
                    } else if chunk.ks == 256 && chunk.iter == 10000 && chunk.ts == 64 {
                        
                    } else {
                        
                    }
                    Err(SjclError::NotImplementedError)
                },
                _ => Err(SjclError::NotImplementedError)
            }
        },
        _ => Err(SjclError::NotImplementedError)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
