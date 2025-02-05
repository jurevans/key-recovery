pub mod crypto;
pub mod types;

use crypto::{keys, mnemonic, utils};
use types::address;
use zeroize::Zeroize;

const INPUT_ERROR: &str = "Enountered error obtaining input from user!";

fn main() {
    let mut phrase = utils::get_input("Enter mnemonic phrase:").expect(INPUT_ERROR);
    let m = mnemonic::Mnemonic::from_phrase(&phrase);
    phrase.zeroize();

    match m {
        Ok(m) => {
            println!("\nMnemonic is valid, continuing...");
            let hd_path = utils::get_input("Enter derivation path:").expect(INPUT_ERROR);
            println!("Using path: {}\n", &hd_path);

            let mut seed = m
                .to_seed(None)
                .expect("Conversion to seed bytes should not fail!");
            let wallet = keys::HDWallet::new(seed.clone()).expect("Could not instantiate wallet!");
            seed.zeroize();

            let keypair = wallet.derive(hd_path).expect("Invalid path!");

            let mut private_key = keypair.private().to_hex();
            println!("Private Key: {}", &private_key);

            let address_util = address::Address::new(private_key.clone());
            private_key.zeroize();

            let public_key = address_util.public();
            let address = address_util.implicit();

            println!("Address: {}", &address);
            println!("Public Key: {}", &public_key);
        }
        Err(error) => {
            panic!("\n{}\n", error);
        }
    }
}
