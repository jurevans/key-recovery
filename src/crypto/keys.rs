use bip32::XPrv;
use thiserror::Error;

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

#[derive(Debug)]
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
#[derive(Debug)]
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

#[derive(Debug)]
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
