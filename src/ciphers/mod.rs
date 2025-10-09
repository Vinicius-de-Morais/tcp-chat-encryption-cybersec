pub mod cesar;
pub mod des;
pub mod monoalphabetic;
pub mod playfair;
pub mod rc4;
pub mod vigenere;

pub trait Cipher {
    // deixei &mut self pq algumas cifras no futuro vão mutar o estado interno (tipo aquelas com rolling key)
    fn to_ciphertext(&mut self, plaintext: &Vec<u8>) -> Vec<u8>;
    fn to_plaintext(&mut self, ciphertext: &Vec<u8>) -> Vec<u8>;
}
