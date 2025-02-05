use std::{fs, process::exit};
use zeroize::Zeroize;

// use crate::crypto::aead;
use serde::Deserialize;
use toml::{self, Value};

use crate::crypto::{
    aead::AEAD,
    utils::{get_input, prompt_password},
};

#[derive(Debug, Deserialize)]
struct Wallet {
    #[allow(dead_code)]
    view_keys: Value,

    #[allow(dead_code)]
    spend_keys: Value,

    #[allow(dead_code)]
    secret_keys: Value,

    #[allow(dead_code)]
    payment_addrs: Value,

    #[allow(dead_code)]
    public_keys: Value,

    #[allow(dead_code)]
    addresses: Value,

    #[allow(dead_code)]
    derivation_paths: Value,

    #[allow(dead_code)]
    pkhs: Value,

    #[allow(dead_code)]
    address_vp_types: Value,
}

pub fn import() {
    println!("Parsing wallet.toml...");
    let filename = "wallet.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(c) => {
            println!("Loaded contents...");
            c
        }
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    let account = get_input("\nWhich account would you like to recover? ").expect("Invalid input");

    let data: Wallet = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Unable to load data from `{}`: {}", filename, err);
            exit(1);
        }
    };

    // TODO: Better checking and error handling here:
    if data.secret_keys[account.clone()].is_str() {
        let entry_maybe = &data.secret_keys[account].as_str();

        if let Some(entry) = entry_maybe {
            let entry_parts = entry.split(":").collect::<Vec<&str>>();
            let prefix = entry_parts.first().expect("Must have a prefix");
            let secret = entry_parts.get(1).expect("Must have a key");

            if prefix == &"encrypted" {
                println!("Found encrypted secret: {}\n", &secret);
                let mut password = prompt_password("Enter your password to decrypt: ");
                let decoded = hex::decode(secret).expect("Hex decoding should not fail!");
                let decrypted = AEAD::decrypt(decoded, password.clone());
                password.zeroize();

                match decrypted {
                    Ok(value) => {
                        println!("Recovered secret: {}", &value);
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            } else {
                println!("Found unencrypted secret: {}", &secret);
            }
        }
    }
}
