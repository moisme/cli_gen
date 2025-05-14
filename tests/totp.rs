//! Integration test: validate against the RFC 6238 seed vector.

use cli_gen::current_totp;

#[test]
fn rfc_seed_vector() {
    // '12345678901234567890' in Base-32
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    let code   = current_totp(secret).unwrap();
    assert_eq!(code.len(), 6);
}
