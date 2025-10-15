use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cipher {
    Caesar,
    MonoalphabeticSubstitution,
    Playfair,
    Vigenere,
    Rc4,
    Des,
}

pub const CIPHERS: [Cipher; 6] = [
    Cipher::Caesar,
    Cipher::MonoalphabeticSubstitution,
    Cipher::Playfair,
    Cipher::Vigenere,
    Cipher::Rc4,
    Cipher::Des,
];

impl ToString for Cipher {
    fn to_string(&self) -> String {
        match self {
            Cipher::Caesar => "Caesar".to_string(),
            Cipher::MonoalphabeticSubstitution => "Monoalphabetic Substitution".to_string(),
            Cipher::Playfair => "Playfair".to_string(),
            Cipher::Vigenere => "Vigenere".to_string(),
            Cipher::Rc4 => "Rc4".to_string(),
            Cipher::Des => "Des".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    author_username: String,
    cipher: Cipher,
    content: Vec<u8>,
}
