use password_generator;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::process;

fn main() {
    let passphrase = password_generator::generate_passphrase();
    println!("Your passphrase is: {}", passphrase);

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(passphrase).unwrap();
    println!("Copied to clipboard! ✅");
    process::exit(0);
}
