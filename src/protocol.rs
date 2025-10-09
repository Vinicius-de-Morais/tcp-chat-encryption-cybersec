use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Cipher {
    Caesar,
    MonoalphabeticSubstitution,
    Playfair,
    Vigenere,
    Rc4,
    Des,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    author_username: String,
    cipher: Cipher,
    content: Vec<u8>,
}
