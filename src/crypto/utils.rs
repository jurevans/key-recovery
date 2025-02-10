use orion::kdf;

/// Encryption salt
pub fn encryption_salt() -> kdf::Salt {
    kdf::Salt::default()
}

/// Derive an encryption key from a password.
pub fn encryption_key(salt: &kdf::Salt, password: String) -> kdf::SecretKey {
    kdf::Password::from_slice(password.as_bytes())
        .and_then(|password| kdf::derive_key(&password, salt, 3, 1 << 17, 32))
        .expect("Generation of encryption secret key shouldn't fail")
}
