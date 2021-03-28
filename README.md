# sjcl-rust
Library which supports decrypting data that was encrypted using SJCL.

## Features
- [x] `AES-CCM`
- [ ] `AES-OCB2`

## Usage
`Cargo.toml`:
```toml
sjcl = "0.0.1"
```

```rust
use sjcl::decrypt_raw;

let data = "{\"iv\":\"nJu7KZF2eEqMv403U2oc3w==\", \"v\":1, \"iter\":10000, \"ks\":256, \"ts\":64, \"mode\":\"ccm\", \"adata\":\"\", \"cipher\":\"aes\", \"salt\":\"mMmxX6SipEM=\", \"ct\":\"VwnKwpW1ah5HmdvwuFBthx0=\"}".to_string();
let password_phrase = "abcdefghi".to_string();

let plaintext = decrypt_raw(data, password_phrase).?;
```
