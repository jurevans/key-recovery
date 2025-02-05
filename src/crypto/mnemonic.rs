use bip0039::Count;

#[derive(Copy, Clone)]
pub enum PhraseSize {
    N12 = 12,
    N24 = 24,
}

#[derive(Debug)]
pub struct Mnemonic {
    phrase: String,
}

impl Mnemonic {
    pub fn new(size: PhraseSize) -> Result<Mnemonic, String> {
        let count: Count = match size {
            PhraseSize::N12 => Count::Words12,
            PhraseSize::N24 => Count::Words24,
        };
        let mnemonic = bip0039::Mnemonic::generate(count);

        Ok(Mnemonic {
            phrase: mnemonic.to_string(),
        })
    }
    pub fn validate(phrase: &str) -> bool {
        bip0039::Mnemonic::validate(phrase).is_ok()
    }

    pub fn from_phrase(phrase: &str) -> Result<Mnemonic, String> {
        if !Mnemonic::validate(phrase) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_mnemonic_from_size() {
        let mnemonic =
            Mnemonic::new(PhraseSize::N12).expect("Should generate mnemonic of length 12");
        let split = mnemonic.phrase.split(' ');
        let words: Vec<&str> = split.collect();

        assert_eq!(words.iter().len(), 12);

        let mnemonic =
            Mnemonic::new(PhraseSize::N24).expect("Should generate mnemonic of length 24");
        let split = mnemonic.phrase.split(' ');
        let words: Vec<&str> = split.collect();

        assert_eq!(words.iter().len(), 24);
    }

    #[test]
    fn can_generate_seed_from_phrase() {
        let phrase = "caught pig embody hip goose like become worry face oval manual flame \
                      pizza steel viable proud eternal speed chapter sunny boat because view bullet";
        let mnemonic = Mnemonic::from_phrase(phrase).unwrap();
        let seed = mnemonic
            .to_seed(None)
            .expect("Should return seed from mnemonic phrase");

        assert_eq!(seed.len(), 64);
    }

    #[test]
    #[should_panic]
    fn invalid_phrase_should_panic() {
        let bad_phrase = "caught pig embody hip goose like become";
        let _ = Mnemonic::from_phrase(bad_phrase.into()).expect("This should fail");
    }
}
