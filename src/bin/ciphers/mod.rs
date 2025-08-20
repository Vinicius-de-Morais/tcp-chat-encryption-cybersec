pub mod playfair;
pub mod vigenere;
pub mod cesar;
pub mod monoalphabetic;

pub trait Cipher {
    // deixei &mut self pq algumas cifras no futuro vão mutar o estado interno (tipo aquelas com rolling key)
    fn to_ciphertext(&mut self, plaintext: &String) -> String;
    fn to_plaintext(&mut self, ciphertext: &String) -> String;
}
