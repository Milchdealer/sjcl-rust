# sjcl-rust
Library which supports decrypting data that was encrypted using SJCL.

## Features
- [x] `AES-CCM`
    - [x] 128-bit keys (`>=0.0.2`)
    - [x] 192-bit keys (`>=0.0.3`)
    - [x] 256-bit keys (`>=0.0.1`)
- [ ] `AES-OCB2`
    - ⚠️OCB2 is deprecated

## Usage
```rust
use sjcl::decrypt_raw;

let data = "{\"iv\":\"nJu7KZF2eEqMv403U2oc3w==\", \"v\":1, \"iter\":10000, \"ks\":256, \"ts\":64, \"mode\":\"ccm\", \"adata\":\"\", \"cipher\":\"aes\", \"salt\":\"mMmxX6SipEM=\", \"ct\":\"VwnKwpW1ah5HmdvwuFBthx0=\"}".to_string();
let password_phrase = "abcdefghi".to_string();

let plaintext = decrypt_raw(data, password_phrase)?;
```
