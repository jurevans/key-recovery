use std::io::{self, Error};

pub fn get_input(prompt: &str) -> Result<String, Error> {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => return Err(err),
    };
    Ok(input.trim().to_string())
}
