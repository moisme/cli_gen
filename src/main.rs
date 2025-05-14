//! Command-line front-end for **cli_gen**.
//!
//! Reads a Base-32 secret from a file, prints the 6-digit TOTP, and
//! optionally copies it to the clipboard or shows an ASCII QR code.

use clap::Parser;
use cli_gen::current_totp;
use std::{fs, path::PathBuf};

/// CLI flag definition.
///
/// Run with `--help` to see the generated help message.
#[derive(Parser)]
#[command(author, version, about = "Generate 6-digit TOTP codes")]
struct Cli {
    /// Path to secret; defaults to ~/.totp-secret
    #[arg(short, long)]
    secret: Option<PathBuf>,

    /// Copy code to clipboard (requires `--features clipboard`)
    #[arg(long)]
    copy: bool,

    /// Emit ASCII QR code (requires `--features qr`)
    #[arg(long)]
    qr: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // ── 1. Read secret ────────────────────────────────────────────────────
    let path = cli
        .secret
        .unwrap_or(dirs::home_dir().unwrap().join(".totp-secret"));
    let secret = fs::read_to_string(path)?;

    // ── 2. Compute the code ───────────────────────────────────────────────
    let code = current_totp(&secret)?;

    // ── 3. Optional clipboard copy ────────────────────────────────────────
    if cli.copy {
        #[cfg(feature = "clipboard")]
        {
            let mut cb = arboard::Clipboard::new()?;
            cb.set_text(code.clone())?;
        }
        #[cfg(not(feature = "clipboard"))]
        eprintln!("recompile with --features clipboard for this flag");
    }

    // ── 4. Optional ASCII QR output ───────────────────────────────────────
    if cli.qr {
        #[cfg(feature = "qr")]
        {
            use qrcodegen::{QrCode, QrCodeEcc};
            let uri = format!("otpauth://totp/rust?secret={secret}");
            let qr  = QrCode::encode_text(&uri, QrCodeEcc::Low)?;

            // Render with a 2-cell border
            let border = 2;
            let size   = qr.size();
            for y in -border..size + border {
                let mut line = String::new();
                for x in -border..size + border {
                    let filled = if x < 0 || y < 0 || x >= size || y >= size {
                        false
                    } else {
                        qr.get_module(x, y)
                    };
                    line.push_str(if filled { "██" } else { "  " });
                }
                println!("{line}");
            }
        }
        #[cfg(not(feature = "qr"))]
        eprintln!("recompile with --features qr for this flag");
    }

    // ── 5. Always print the code last ─────────────────────────────────────
    println!("{code}");
    Ok(())
}
