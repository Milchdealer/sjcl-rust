# sjcl-rust
Library which supports decrypting and encrypting SJCL compatible blocks.

## Features
- Encryption & Decryption
- Key sizes between 128- to 256-bit
- `AES-CCM`
- ⚠️ `AES-OCB2` is deprecated and **not** supported

## Usage
```rust
use sjcl::decrypt_raw;

let data = "{\"iv\":\"nJu7KZF2eEqMv403U2oc3w==\", \"v\":1, \"iter\":10000, \"ks\":256, \"ts\":64, \"mode\":\"ccm\", \"adata\":\"\", \"cipher\":\"aes\", \"salt\":\"mMmxX6SipEM=\", \"ct\":\"VwnKwpW1ah5HmdvwuFBthx0=\"}".to_string();
let password_phrase = "abcdefghi".to_string();

let plaintext = String::from_utf8(decrypt_json(data, password_phrase, None).unwrap())?;
```
