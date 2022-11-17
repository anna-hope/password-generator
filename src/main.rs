use std::error::Error;

use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use password_generator::SeparatorKind;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = SeparatorKind::Hyphen)]
    separator: SeparatorKind,
    #[arg(short, long)]
    add_special_characters: bool,
    #[arg(short, long)]
    capitalize: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let passphrase = password_generator::generate_passphrase(
        args.separator,
        args.capitalize,
        args.add_special_characters,
    );
    println!("Your passphrase is: {}", passphrase);

    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(passphrase)?;
    println!("Copied to clipboard! ✅");
    Ok(())
}
