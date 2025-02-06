use orion::kdf;
use std::io::{self, Error, Write};
extern crate rpassword;

use rpassword::read_password;

pub fn get_input(prompt: &str) -> Result<String, Error> {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => return Err(err),
    };
    Ok(input.trim().to_string())
}

pub fn prompt_password(prompt: &str) -> String {
    print!("{}", &prompt);
    std::io::stdout().flush().unwrap();
    read_password().unwrap()
}

/// Encryption salt
pub fn encryption_salt() -> kdf::Salt {
    kdf::Salt::default()
}

/// Derive an encryption key from a password.
pub fn encryption_key(salt: &kdf::Salt, password: String) -> kdf::SecretKey {
    kdf::Password::from_slice(password.as_bytes())
        .and_then(|password| kdf::derive_key(&password, salt, 3, 1 << 17, 32))
        .expect("Generation of encryption secret key shouldn't fail")
}
