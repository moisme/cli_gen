mod library;

use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use crate::library::current_totp;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    /// Path to secret; defaults to ~/.totp-secret
    #[arg(short, long)]
    secret: Option<PathBuf>,

    /// Copy code to clipboard (requires `--features clipboard`)
    #[arg(long)]
    copy: bool,

    /// Emit ASCII QR code for the secret (requires `--features qr`)
    #[arg(long)]
    qr: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. read secret ---------------------------------------------------------
    let path = cli
        .secret
        .unwrap_or(dirs::home_dir().unwrap().join(".totp-secret"));
    let secret = fs::read_to_string(path)?;

    // 2. compute code --------------------------------------------------------
    let code = current_totp(&secret)?;

    // 3. copy to clipboard if requested --------------------------------------
    if cli.copy {
        #[cfg(feature = "clipboard")]
        {
            let mut cb = arboard::Clipboard::new()?;
            cb.set_text(code.clone())?;
        }
        #[cfg(not(feature = "clipboard"))]
        eprintln!("recompile with --features clipboard for this flag");
    }

    // 4. optional QR ---------------------------------------------------------
    if cli.qr {
        #[cfg(feature = "qr")]
        {
            use qrcodegen::{QrCode, QrCodeEcc};
            let uri = format!("otpauth://totp/rust?secret={secret}");
            let qr = QrCode::encode_text(&uri, QrCodeEcc::Low)?;
            println!("{}", qr.to_string(true, 2)); // ASCII art
        }
        #[cfg(not(feature = "qr"))]
        eprintln!("recompile with --features qr for this flag");
    }

    // 5. print the code last -------------------------------------------------
    println!("{code}");
    Ok(())
}

