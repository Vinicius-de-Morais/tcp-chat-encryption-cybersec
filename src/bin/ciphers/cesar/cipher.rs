use crate::ciphers::Cipher;

pub struct Cesar {
    key: i8, // chave de deslocamento
}

impl Cesar {
    pub fn new(key: i8) -> Self {
        return Cesar { key };
    }

    pub fn process(&self, text: &str, encrypt: bool) -> String {
        let mut out = String::with_capacity(text.len());
        let shift = if encrypt { self.key } else { -self.key };

        for ch in text.chars() {
            if ch.is_ascii_alphabetic() {
                let a: u8 = if ch.is_ascii_uppercase() { b'A' } else { b'a' };
                let alpha_index = (ch as u8 - a) as i8;
                let shifted = (alpha_index + shift + 26) % 26; // +26 para evitar negativo
                out.push((a + shifted as u8) as char);
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn encrypt(&self, plaintext: &String) -> String {
        self.process(plaintext, true)
    }

    fn decrypt(&self, ciphertext: &String) -> String {
        self.process(ciphertext, false)
    }
}

impl Cipher for Cesar {
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
        let mut v = Cesar::new(3);
        let cipher = v.to_ciphertext(&"EVIDENCIAS".to_string());
        assert_eq!(cipher, "HYLGHQFLDV");
        let plain = v.to_plaintext(&cipher);
        assert_eq!(plain, "EVIDENCIAS");
    }
}
