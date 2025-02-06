use std::{fs, process::exit};
use zeroize::Zeroize;

// use crate::crypto::aead;
use serde::Deserialize;
use toml::{self, Value};

use crate::{
    crypto::aead::AEAD,
    types::address,
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

/// Export secret key from wallet.toml
pub fn export() {
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

    let account =
        get_input("\nEnter the alias of the account to recover: ").expect("Invalid input");

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
            let secret = entry_parts.get(1).expect("Must have a secret");

            if prefix == &"encrypted" {
                println!("Found encrypted secret: {}\n", &secret);
                let mut password = prompt_password("Enter your password to decrypt: ");
                let decoded = hex::decode(secret).expect("Hex decoding should not fail!");
                let decrypted = AEAD::decrypt(decoded, password.clone());
                password.zeroize();

                match decrypted {
                    Ok(private_key_bytes) => {
                        let mut private_key = hex::encode(private_key_bytes);
                        println!("\nRecovered secret! {}\n", &private_key);
                        let address_util =
                            address::Address::new(private_key.clone()[2..].to_string());
                        private_key.zeroize();

                        let public_key = address_util.public();
                        let address = address_util.implicit();

                        println!("Address: {}", &address);
                        println!("Public Key: {}", &public_key);
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
