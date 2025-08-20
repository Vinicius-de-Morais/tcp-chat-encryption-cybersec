use crate::ciphers::{playfair::helper::Unique, Cipher};

pub struct Monoalphabetic {
    mapping: Vec<char>,
}

impl Monoalphabetic {
    fn to_plaintext(&mut self, ciphertext: &String) -> String {
        self.process(ciphertext, false)
    }

    fn to_ciphertext(&mut self, plaintext: &String) -> String {
        self.process(plaintext, true)
    }

    pub fn new(key: String) -> Self {
        let mut mapping: Vec<char> = key
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase())
            .collect();
        mapping.unique();
        assert!(mapping.len() == 26, "chave monoalfabética inválida");
        Monoalphabetic { mapping }
    }

    fn process(&self, text: &str, encrypt: bool) -> String {
        let mut out = String::with_capacity(text.len());
        let alphabet: Vec<char> = (b'A'..=b'Z').map(|c| c as char).collect();

        for ch in text.chars() {
            if ch.is_ascii_alphabetic() {
                let is_upper = ch.is_ascii_uppercase();
                let upper_ch = ch.to_ascii_uppercase();
                let idx = if encrypt {
                    self.mapping
                        .iter()
                        .position(|&c| c == upper_ch)
                        .unwrap_or_else(|| panic!("caractere {} não encontrado na chave", ch))
                } else {
                    alphabet
                        .iter()
                        .position(|&c| c == upper_ch)
                        .unwrap_or_else(|| panic!("caractere {} não encontrado no alfabeto", ch))
                };
                let mapped_char = if encrypt {
                    alphabet[idx]
                } else {
                    self.mapping[idx]
                };
                out.push(if is_upper {
                    mapped_char
                } else {
                    mapped_char.to_ascii_lowercase()
                });
            } else {
                out.push(ch);
            }
        }
        out
    }
}

impl Cipher for Monoalphabetic {
    fn to_ciphertext(&mut self, plaintext: &String) -> String {
        self.process(plaintext, true)
    }

    fn to_plaintext(&mut self, ciphertext: &String) -> String {
        self.process(ciphertext, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_example() {
        // Exemplo clássico: TEXTO: ATTACKATDAWN, KEY: LEMON -> LXFOPVEFRNHR
        let mut v = Monoalphabetic::new("CTOGDRHZBFUYLKNEAJWMQPXSIV".to_string());
        let cipher = v.to_ciphertext(&"ATHLETICO".to_string());
        assert_eq!(cipher, "CMZYDMBON");
        let plain = v.to_plaintext(&cipher);
        assert_eq!(plain, "ATHLETICO");
    }
}
