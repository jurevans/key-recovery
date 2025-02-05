use orion::{aead, kdf};
use std::str;
use thiserror::Error;

use crate::crypto::utils::{encryption_key, encryption_salt};

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum DecryptionError {
    #[error("Unexpected encryption salt")]
    BadSalt,
    #[error("Unable to decrypt. Is the password correct?")]
    DecryptionError,
}

pub struct AEAD;

/// A set of simple WASM-compatible AEAD (Authenticated Encryption with Additional Data) functions
impl AEAD {
    pub fn encrypt_from_bytes(bytes: Vec<u8>, password: String) -> Vec<u8> {
        let bytes: &[u8] = &bytes;
        let salt = encryption_salt();
        let encryption_key = encryption_key(&salt, password);
        let encrypted_keypair =
            aead::seal(&encryption_key, bytes).expect("Encryption of data shouldn't fail");

        [salt.as_ref(), &encrypted_keypair].concat()
    }

    pub fn encrypt(value: String, password: String) -> Vec<u8> {
        let data = Vec::from(value.as_bytes());
        AEAD::encrypt_from_bytes(data, password)
    }

    pub fn decrypt(encrypted: Vec<u8>, password: String) -> Result<String, String> {
        let salt_len = encryption_salt().len();
        let (raw_salt, cipher) = encrypted.split_at(salt_len);

        let salt =
            kdf::Salt::from_slice(raw_salt).map_err(|_| DecryptionError::BadSalt.to_string())?;

        let encryption_key = encryption_key(&salt, password);

        let decrypted_data: &[u8] = &aead::open(&encryption_key, cipher)
            .map_err(|err| format!("{}: {:?}", &DecryptionError::DecryptionError, err))?;

        let s = match str::from_utf8(decrypted_data) {
            Ok(v) => v,
            Err(error) => return Err(error.to_string()),
        };

        Ok(String::from(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_decrypt_encrypted_string() {
        let password = String::from("unhackable");
        let message = String::from("My secret message");

        let encrypted = AEAD::encrypt(message.clone(), password.clone());
        let decrypted = AEAD::decrypt(encrypted, password).expect("Value should be decrypted");

        assert_eq!(decrypted, message);
    }

    #[test]
    fn can_decrypt_encrypted_data() {
        let password = String::from("unhackable");
        let message = String::from("My secret message");
        let bytes = Vec::from(message.as_bytes());

        let encrypted = AEAD::encrypt_from_bytes(bytes, password.clone());
        let decrypted = AEAD::decrypt(encrypted, password).expect("Value should be decrypted");

        assert_eq!(decrypted, message);
    }
}
