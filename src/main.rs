pub mod cli;
pub mod crypto;
pub mod types;
pub mod utils;

fn main() {
    // Derive keys from mnemonic:
    // cli::hdkeys::derive();

    // Export secret key from wallet.toml
    cli::export::export();
}
