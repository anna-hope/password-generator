use password_generator;

fn main() {
    let wordlist_contents = password_generator::get_wordlist_data();
    let roll2word = password_generator::parse_diceware_wordlist(wordlist_contents);
    println!("{}", roll2word.len());
}
