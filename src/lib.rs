//! Minimal library crate for the **cli_gen** project.
//!
//! Exposes a single helper—[`current_totp`]—that converts a Base-32
//! secret into the 6-digit TOTP code Google Authenticator would show.

use base32::Alphabet::RFC4648;
use totp_rs::{Algorithm, TOTP};

/// Number of digits in the resulting TOTP code.
const DIGITS: u32 = 6;

/// Compute the **current** 6-digit TOTP code for the given Base-32
/// secret.
///
/// * `secret_b32` – Secret in RFC 4648 Base-32, **without** trailing
///   padding (`=`).
///
/// # Errors
///
/// * Returns `Err` if the secret cannot be decoded from Base-32.
///
/// # Example
///
/// ```
/// let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
/// let code   = cli_gen::current_totp(secret)?;
/// assert_eq!(code.len(), 6);
/// # Ok::<_, anyhow::Error>(())
/// ```
pub fn current_totp(secret_b32: &str) -> anyhow::Result<String> {
    // 1. Decode Base-32 → raw bytes
    let key = base32::decode(RFC4648 { padding: false }, secret_b32.trim())
        .ok_or_else(|| anyhow::anyhow!("invalid Base-32 secret"))?;

    // 2. Build a TOTP generator (30 s period, SHA-1, 6 digits)
    let totp = TOTP::new(Algorithm::SHA1, DIGITS as usize, 1, 30, key);

    // 3. Generate and return the code
    Ok(totp?.generate_current()?.to_string())
}
