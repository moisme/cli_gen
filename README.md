cli_gen – TOTP‑CLI in Rust
cli_gen is a lightweight command‑line tool that generates 
the exact 6‑digit TOTP (Time‑based One‑Time Password) codes you would see 
in an authenticator app. The project is intentionally small—perfect 
demo material for showcasing idiomatic Rust, conditional compilation, and basic cryptography 
without external hardware.

Features

Flag      Description            Default
(none)    Print the current 
          6‑digit TOTP 
          to stdout.              YES
--secret 
<PATH> 
or -s      Read secret from 
          a custom path instead
          of ~/.totp‑secret.        –
--copy ⋯ 
clipboard  Copy the code to the
          system clipboard 
          (requires feature).
                                    off
--qr ⋯ 
qr        Render an ASCII QR 
          code that you can 
          scan into any 
          authenticator.
                                    off

Feature flags
 • clipboard → adds the arboard crate for cross‑platform copy
 • qr → adds the qrcodegen crate and ASCII renderer

Quick start
Clone & build
 git clone …/cli_gen && cd cli_gen
cargo run --release --  # reads ~/.totp-secret and prints the code


Optional: Clipboard copy only: 

	cargo run --release --features clipboard -- --secret /tmp/demo-secret --copy pbpaste

Optional: clipboard copy + QR rendering

cargo run --features "clipboard qr" -- \ --secret /tmp/demo-secret --qr
Secret file format
Plain Base‑32 text, no padding—exactly the value you’d copy from a provisioning QR‑code URI. Example (RFC 6238 test vector):
GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ
Store it at ~/.totp-secret or pass --secret /path/to/file.

Running the tests
$ cargo test
The included integration test uses the official RFC 6238 seed to assert that the current_totp() function returns a 6‑digit string.

Here is an example output with the secret code at the bottom, ability to paste the code as it is copied to your clipboard and the QR code on top 

<img width="769" alt="Screenshot 2025-05-14 at 04 29 02" src="https://github.com/user-attachments/assets/0ca64ee8-0026-4f0d-be1f-a7f29c922f86" />



