pub mod cesar;
pub mod monoalphabetic;
pub mod playfair;
pub mod rc4;
pub mod rc4_bortoli;
pub mod vigenere;

pub trait Cipher {
    // deixei &mut self pq algumas cifras no futuro vão mutar o estado interno (tipo aquelas com rolling key)
    fn to_ciphertext(&mut self, plaintext: &String) -> String;
    fn to_plaintext(&mut self, ciphertext: &String) -> String;
}
