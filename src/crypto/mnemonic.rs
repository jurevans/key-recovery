#[derive(Debug)]
pub struct Mnemonic {
    phrase: String,
}

impl Mnemonic {
    pub fn validate(phrase: &str) -> bool {
        bip0039::Mnemonic::validate(phrase).is_ok()
    }

    pub fn from_phrase(phrase: &str) -> Result<Mnemonic, String> {
        if Mnemonic::validate(phrase) == false {
            return Err(String::from("Invalid mnemonic phrase!"));
        }
        Ok(Mnemonic {
            phrase: phrase.to_string(),
        })
    }

    pub fn to_seed(&self, passphrase: Option<String>) -> Result<Vec<u8>, String> {
        let passphrase = match passphrase {
            Some(passphrase) => passphrase,
            None => "".to_string(),
        };
        let mnemonic = match bip0039::Mnemonic::from_phrase(self.phrase.clone()) {
            Ok(mnemonic) => mnemonic,
            Err(_) => return Err(String::from("Unable to parse mnemonic!")),
        };
        let seed: &[u8] = &mnemonic.to_seed(&passphrase);

        Ok(Vec::from(seed))
    }
}
