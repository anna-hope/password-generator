use password_generator;

fn main() {
    let passphrase = password_generator::generate_passphrase();
    println!("{}", passphrase);
}
