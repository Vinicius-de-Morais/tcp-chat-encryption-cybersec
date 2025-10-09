use crate::ciphers::Cipher;

pub mod bit;
pub mod cipher;
pub mod tables;

pub struct DES {
    pub key: u64,
}

impl DES {
    pub fn new(key: &Vec<u8>) -> Self {
        let keyu64 = u64::from_be_bytes(DES::vec_to_array(key));

        DES { key: keyu64 }
    }

    fn vec_to_array(vec: &Vec<u8>) -> [u8; 8] {
        let mut array: [u8; 8] = [0; 8];

        for (i, &value) in vec.iter().enumerate().take(8) {
            array[i] = value;
        }

        array
    }
}

impl Cipher for DES {
    fn to_ciphertext(&mut self, plaintext: &Vec<u8>) -> Vec<u8> {
        let input = u64::from_be_bytes(DES::vec_to_array(plaintext));
        let output = cipher::process(input, self.key, cipher::DESMode::Cipher);

        output.to_be_bytes().to_vec()
    }

    fn to_plaintext(&mut self, ciphertext: &Vec<u8>) -> Vec<u8> {
        let input = u64::from_be_bytes(DES::vec_to_array(ciphertext));
        let output = cipher::process(input, self.key, cipher::DESMode::Decipher);

        output.to_be_bytes().to_vec()
    }
}
