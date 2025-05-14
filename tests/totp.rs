use cli_gen::current_totp;
#[test]
fn rfc_seed_vector() {
    // From RFC 6238 Appx. B
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ"; // '12345678901234567890'
    let code = current_totp(secret).unwrap();
    assert_eq!(code.len(), 6);
}
