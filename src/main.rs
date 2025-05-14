use cli_gen::current_totp;

use clap::{Parser};
use std::{fs, path::PathBuf};

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
            let border: i32 = 2;
            let size       = qr.size();
            for y in -border..size + border {
                let mut line = String::new();
                for x in -border..size + border {
                    let filled = if x < 0 || y < 0 || x >= size || y >= size {
                        false                     // outer white border
                    } else {
                        qr.get_module(x, y)       // true = dark
                    };
                    line.push_str(if filled { "██" } else { "  " });
                }
                println!("{line}");
            }
            // ----------------------
        }
        #[cfg(not(feature = "qr"))]
        eprintln!("recompile with --features qr for this flag");
    }

    // 5. print the code last -------------------------------------------------
    println!("{code}");
    Ok(())
}

