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

/// Helper function to bech32 encode a public key from bytes
pub fn public_key_to_bech32(bytes: Vec<u8>) -> Result<String, Error> {
    let public_key = PublicKey::try_from_slice(&bytes)?;

    Ok(public_key.to_string())
}

pub struct Address {
    implicit: address::Address,
    public: PublicKey,
    hash: PublicKeyHash,
}

impl Address {
    pub fn new(secret: String) -> Address {
        let private = SecretKey::Ed25519(
            key::ed25519::SecretKey::from_str(&secret).expect("ed25519 encoding should not fail"),
        );

        #[allow(clippy::useless_conversion)]
        let public = PublicKey::from(private.ref_to());
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
