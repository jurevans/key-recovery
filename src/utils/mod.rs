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
