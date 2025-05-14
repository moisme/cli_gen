use chrono::Utc; //This imports the Utc time source
use totp_rs::{Algorithm, Secret, TOTP};
use base32::Alphabet::RFC4648;


const DIGITS: u32 = 6;        // 6-digit codes

pub fn current_totp(secret_b32: &str) -> anyhow::Result<String> {
    // Decodes Base32 → Raw bytes
    let key = base32::decode(RFC4648 { padding: false }, secret_b32.trim())
        .ok_or_else(|| anyhow::anyhow!("invalid Base-32 secret"))?;


    // 30-second TOTP, same as Google Authenticator (HMAC-SHA-1)
    // 2. build a TOTP generator (30-second period, SHA-1, 6 digits)
    let totp = TOTP::new(Algorithm::SHA1, DIGITS as usize, 1, 30, key);

    // 3. generate the current code
    Ok(totp?.generate_current()?.to_string())
}