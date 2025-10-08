use crate::ciphers::{des, Cipher};

pub mod bit;
pub mod cipher;
pub mod tables;

pub struct DES {
    pub key: u64,
}

impl DES {
    pub fn new(key: &str) -> Self {
        let keyu64 = DES::interpret_string_as_64_bit_hex(&key);

        DES { key: keyu64 }
    }

    fn interpret_string_as_64_bit_hex(input: &str) -> u64 {
        assert!(input.len() <= 18, "string de tamanho inválido");

        let mut cleaned = input.to_string().to_lowercase();

        if cleaned.starts_with("0x") {
            cleaned.drain(0..2);
        }

        u64::from_str_radix(&cleaned, 16).unwrap()
    }
}

impl Cipher for DES {
    fn to_ciphertext(&mut self, plaintext: &String) -> String {
        let input = DES::interpret_string_as_64_bit_hex(&plaintext);
        let output = cipher::process(input, self.key, cipher::DESMode::Cipher);

        format!("{:X}", output)
    }

    fn to_plaintext(&mut self, ciphertext: &String) -> String {
        let input = DES::interpret_string_as_64_bit_hex(&ciphertext);
        let output = cipher::process(input, self.key, cipher::DESMode::Decipher);

        format!("{:X}", output)
    }
}
