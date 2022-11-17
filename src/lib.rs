use directories::ProjectDirs;
use rand::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::path;

#[macro_use]
extern crate lazy_static;

const DICEWARE_URL: &str = "https://theworld.com/~reinhold/diceware.wordlist.asc";
const WORDLIST_FILENAME: &str = "wordlist";
const SPECIAL_CHARACTERS: &[&str] = &["!", "\"", "#", "$", "%", "&", "\'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~"];

#[derive(clap::ValueEnum, Clone)]
pub enum SeparatorKind {
    Digit,
    Hyphen,
}

fn get_data_dir() -> path::PathBuf {
    let project_dirs = ProjectDirs::from("me", "annahope", "password_generator").unwrap();
    let data_dir = project_dirs.data_dir().to_str().unwrap();
    path::PathBuf::from(data_dir)
}

fn create_wordlist_parent_path(data_dir: &path::PathBuf) -> io::Result<()> {
    if !data_dir.exists() {
        return fs::create_dir_all(data_dir);
    }
    Ok(())
}

fn get_wordlist_data_path(data_dir: &path::PathBuf) -> String {
    let data_dir_str = data_dir.to_str().unwrap();
    format_args!(
        "{data_dir}{sep}{filename}",
        data_dir = data_dir_str,
        sep = path::MAIN_SEPARATOR,
        filename = WORDLIST_FILENAME
    )
    .to_string()
}

fn download_diceware_wordlist(wordlist_path: &str) -> Result<u64, &'static str> {
    println!("Downloading the wordlist to {}", wordlist_path);

    let resp = reqwest::blocking::get(DICEWARE_URL).expect("could not download the DiceWare file");
    let body = resp
        .text()
        .expect("The body of the downloaded file is invalid");
    let mut out = File::create(&wordlist_path).expect("Failed to create file");

    if let Ok(bytes_written) = io::copy(&mut body.as_bytes(), &mut out) {
        return Ok(bytes_written);
    } else {
        return Err("could not download the wordlist");
    }
}

fn get_wordlist_data() -> String {
    let data_dir = get_data_dir();
    create_wordlist_parent_path(&data_dir).expect("Could not create the parent directory");

    let wordlist_path = get_wordlist_data_path(&data_dir);
    if !path::Path::new(&wordlist_path).exists() {
        download_diceware_wordlist(&wordlist_path).expect("Couldn't get the wordlist file");
    }
    fs::read_to_string(&wordlist_path).expect("Couldn't read the wordlist file")
}

fn parse_wordlist_line(line: &str) -> Option<(&str, &str)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)\t(\S+)").unwrap();
    }
    if let Some(captures) = RE.captures(line) {
        let rolls = captures.get(1).unwrap().as_str();
        let word = captures.get(2).unwrap().as_str();
        Some((rolls, word))
    } else {
        None
    }
}

fn parse_diceware_wordlist(wordlist_data: String) -> HashMap<String, String> {
    let mut dice_rolls2word = HashMap::new();
    for line in wordlist_data.lines() {
        if let Some((rolls, word)) = parse_wordlist_line(line) {
            dice_rolls2word.insert(rolls.to_string(), word.to_string());
        }
    }
    dice_rolls2word
}

fn generate_dice_rolls() -> String {
    let mut rng = StdRng::from_entropy();
    // we need 5 dice rolls
    let mut dice_rolls: Vec<u8> = vec![0; 5];
    for i in 0..5 {
        let dice_roll = rng.gen_range(1..=6);
        dice_rolls[i] = dice_roll;
    }
    dice_rolls
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .concat()
}

pub fn generate_passphrase(
    separator_kind: SeparatorKind,
    capitalize: bool,
    add_special_characters: bool,
) -> String {
    let wordlist_contents = get_wordlist_data();
    let roll2word = parse_diceware_wordlist(wordlist_contents);
    let mut tokens = Vec::with_capacity(5);

    for _ in 0..5 {
        let some_dice_roll = generate_dice_rolls();
        let mut word = roll2word.get(&some_dice_roll).unwrap().to_owned();
        if capitalize {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars[0] = chars[0].to_ascii_uppercase();
            word = chars
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("");
        }
        tokens.push(word);
    }

    if add_special_characters {
        let mut rng = StdRng::from_entropy();
        for _ in 0..3 {
            let special_char_index = rng.gen_range(0..SPECIAL_CHARACTERS.len());
            let special_char = SPECIAL_CHARACTERS[special_char_index].to_string();

            let random_index = rng.gen_range(0..tokens.len());
            tokens.insert(random_index, special_char);
        }
    }

    match separator_kind {
        SeparatorKind::Digit => {
            let mut rng = StdRng::from_entropy();
            for i in 0..tokens.len() - 1 {
                let digit = rng.gen_range(0..9);
                tokens.insert(i + i + 1, digit.to_string());
            }
            tokens.join("")
        }
        SeparatorKind::Hyphen => tokens.join("-"),
    }
}
