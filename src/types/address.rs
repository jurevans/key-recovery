use namada_sdk::borsh::BorshDeserialize;
use namada_sdk::{
    address,
    key::{
        self,
        common::{PublicKey, SecretKey},
        PublicKeyHash, RefTo,
    },
};
use std::io::Error;
use std::str::FromStr;
use zeroize::Zeroize;

/// Helper function to bech32 encode a public key from bytes
pub fn public_key_to_bech32(bytes: Vec<u8>) -> Result<String, Error> {
    let public_key = PublicKey::try_from_slice(&bytes)?;

    Ok(public_key.to_string())
}

#[derive(Debug)]
pub struct Address {
    implicit: address::Address,
    public: PublicKey,
    hash: PublicKeyHash,
}

impl Address {
    pub fn new(mut secret: String) -> Address {
        let private = SecretKey::Ed25519(
            key::ed25519::SecretKey::from_str(&secret).expect("ed25519 encoding should not fail"),
        );

        #[allow(clippy::useless_conversion)]
        let public = PublicKey::from(private.ref_to());

        secret.zeroize();

        let hash = PublicKeyHash::from(&public);
        let implicit = address::Address::Implicit(address::ImplicitAddress::from(&public));

        Address {
            implicit,
            public,
            hash,
        }
    }

    pub fn implicit(&self) -> String {
        self.implicit.encode()
    }

    pub fn public(&self) -> String {
        self.public.to_string()
    }

    pub fn hash(&self) -> String {
        self.hash.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_implicit_address() {
        let secret =
            String::from("1498b5467a63dffa2dc9d9e069caf075d16fc33fdd4c3b01bfadae6433767d93");
        let address = Address::new(secret);
        let implicit = address.implicit();

        assert_eq!(implicit, "tnam1qpgk927uh2aqjs92yhycsh08n5yggvltn5nk92zp");
        assert_eq!(implicit.len(), address::ADDRESS_LEN);
    }

    #[test]
    fn can_return_correct_public_key() {
        let secret =
            String::from("1498b5467a63dffa2dc9d9e069caf075d16fc33fdd4c3b01bfadae6433767d93");
        let address = Address::new(secret);
        let public = address.public();

        assert_eq!(
            public,
            "tpknam1qzm68sfdcryvwj9tqaf9kuq3y2ugh4u0vqx8vdpdyle9uhujg3xduf408cn"
        );
    }
}
