use bip32::XPrv;
use thiserror::Error;
use zeroize::ZeroizeOnDrop;

#[derive(Debug, Error)]
pub enum HDWalletError {
    #[error("Unable to parse path")]
    PathError,
    #[error("Unable to derive keys from path")]
    DerivationError,
    #[error("Could not create secret key from bytes")]
    SecretKeyError,
    #[error("Invalid key size")]
    InvalidKeySize,
    #[error("Invalid seed length")]
    InvalidSeed,
}

#[derive(Debug, ZeroizeOnDrop)]
pub struct Key {
    bytes: [u8; 32],
}

/// A 32 byte ed25519 key
impl Key {
    pub fn new(bytes: Vec<u8>) -> Result<Key, String> {
        let bytes: [u8; 32] = match bytes.try_into() {
            Ok(bytes) => bytes,
            Err(err) => return Err(format!("{}: {:?}", HDWalletError::InvalidKeySize, err)),
        };

        Ok(Key { bytes })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        Vec::from(self.bytes)
    }

    pub fn to_hex(&self) -> String {
        let bytes: &[u8] = &self.bytes;
        hex::encode(bytes)
    }
}

/// An ed25519 keypair
#[derive(Debug, ZeroizeOnDrop)]
pub struct Keypair {
    private: Key,
    public: Key,
}

impl Keypair {
    pub fn private(&self) -> Key {
        Key {
            bytes: self.private.bytes,
        }
    }

    pub fn public(&self) -> Key {
        Key {
            bytes: self.public.bytes,
        }
    }
}

#[derive(Debug, ZeroizeOnDrop)]
pub struct HDWallet {
    seed: [u8; 64],
}

/// A set of methods to derive keys from a BIP32/BIP44 path
impl HDWallet {
    pub fn new(seed: Vec<u8>) -> Result<HDWallet, String> {
        let seed: [u8; 64] = match seed.try_into() {
            Ok(seed) => seed,
            Err(err) => return Err(format!("{}: {:?}", HDWalletError::InvalidSeed, err)),
        };

        Ok(HDWallet { seed })
    }

    /// Derive account from a seed and a path
    pub fn derive(&self, path: String) -> Result<Keypair, String> {
        // BIP32 Extended Private Key
        let path = path
            .parse()
            .map_err(|err| format!("{}: {:?}", HDWalletError::PathError, err))?;
        let xprv = XPrv::derive_from_path(self.seed, &path)
            .map_err(|_| HDWalletError::DerivationError.to_string())?;

        let prv_bytes: &mut [u8] = &mut xprv.private_key().to_bytes();

        // ed25519 keypair
        let secret_key = ed25519_dalek::SecretKey::from_bytes(prv_bytes)
            .map_err(|_| HDWalletError::SecretKeyError.to_string())?;
        let public_key = ed25519_dalek::PublicKey::from(&secret_key);

        let private = Key::new(Vec::from(secret_key.to_bytes()))?;
        let public = Key::new(Vec::from(public_key.to_bytes()))?;

        Ok(Keypair { private, public })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::mnemonic::{Mnemonic, PhraseSize};

    #[test]
    fn can_derive_keys_from_path() {
        let phrase = "caught pig embody hip goose like become worry face oval manual flame \
                      pizza steel viable proud eternal speed chapter sunny boat because view bullet";
        let mnemonic = Mnemonic::from_phrase(phrase).expect("Should not fail with a valid phrase!");
        let seed = mnemonic.to_seed(None).unwrap();
        let bip44: HDWallet = HDWallet::new(seed).unwrap();
        // Account (/0'/0) - External path
        let path = "m/44'/0'/0'/0";

        let keys = bip44
            .derive(path.to_string())
            .expect("Should derive keys from a path");

        assert_eq!(keys.private.to_bytes().len(), 32);
        assert_eq!(keys.public.to_bytes().len(), 32);

        let secret_hex = &keys.private.to_hex();
        assert_eq!(
            secret_hex,
            "c620766dca00ed9a345dd9d3aa3caa97caf333594dbdc928b7aa172efa3e27c3"
        );

        let public_hex = &keys.public.to_hex();
        assert_eq!(
            public_hex,
            "27152d5f6f6abfcec1ccc8a5334592051cb2134d04cd64e9aec10698dc5de782"
        );

        // Sub-Account (/0'/0/0) - External path
        let path = "m/44'/0'/0'/0/0";

        let keys = bip44
            .derive(path.to_string())
            .expect("Should derive keys from a path");
        let secret_hex = &keys.private.to_hex();
        assert_eq!(
            secret_hex,
            "2493640b28d0ab262451713fdff14d6fb5e5c4d2652f1e3aba301e23fe5c4442"
        );

        let public_hex = &keys.public.to_hex();
        assert_eq!(
            public_hex,
            "cb312d148b5e615ee2854307b0b0c17133cd24c44f35d3cb554cf29c7b2addb1"
        );
    }

    #[test]
    #[should_panic]
    fn invalid_seed_should_panic() {
        let _bip44 = HDWallet::new(vec![0, 1, 2, 3, 4]).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_key_should_panic() {
        let _key = Key::new(vec![0, 1, 2, 3, 4]).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_derivation_path_should_panic() {
        let m = Mnemonic::new(PhraseSize::N12).expect("New mnemonic should not fail");
        let seed = m.to_seed(None).expect("Mnemonic to seed should not fail");
        let b = HDWallet::new(seed).expect("HDWallet from seed should not fail");

        let bad_path = "m/44/0 '/ 0";
        let _keypair = b.derive(bad_path.to_string()).unwrap();
    }
}
