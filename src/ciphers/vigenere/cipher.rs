use crate::ciphers::Cipher;

pub struct Vigenere {
    key: String,
}

impl Vigenere {
    pub fn new(key: String) -> Self {
        let normalized = key
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>();
        assert!(!normalized.is_empty(), "chave vigenere vazia");
        Vigenere { key: normalized }
    }

    fn process(&self, text: &str, encrypt: bool) -> String {
        let mut out = String::with_capacity(text.len());
        let key_bytes = self.key.as_bytes();
        let mut key_index = 0usize;

        for ch in text.chars() {
            // Ignorar caracteres não alfabéticos
            if ch.is_ascii_alphabetic() {
                let a: u8 = if ch.is_ascii_uppercase() { b'A' } else { b'a' }; // dar match no case
                let key_shift = (key_bytes[key_index % key_bytes.len()] - b'A') as i16;

                // "index" da letra
                let plain_pos = (ch as u8 - a) as i16;
                let shifted: u8;

                // se criptografar trazemos a posição de troca
                // se descriptografar "regredimos" a posição de troca
                if encrypt {
                    shifted = ((plain_pos + key_shift) % 26) as u8;
                } else {
                    shifted = ((plain_pos - key_shift + 26) % 26) as u8;
                }

                // index do alfabeto + posição de troca
                out.push((a + shifted) as char);
                key_index += 1;
            } else {
                // manter caracteres tipo espaço e barra
                out.push(ch);
            }
        }
        out
    }
}

impl Cipher for Vigenere {
    fn to_ciphertext(&mut self, plaintext: &Vec<u8>) -> Vec<u8> {
        let input = String::from_utf8(plaintext.iter().map(|c| *c).collect::<Vec<u8>>()).unwrap();

        self.process(&input, true).as_bytes().to_vec()
    }

    fn to_plaintext(&mut self, ciphertext: &Vec<u8>) -> Vec<u8> {
        let input = String::from_utf8(ciphertext.iter().map(|c| *c).collect::<Vec<u8>>()).unwrap();

        self.process(&input, false).as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_example() {
        // Exemplo clássico: TEXTO: ATTACKATDAWN, KEY: LEMON -> LXFOPVEFRNHR
        let mut v = Vigenere::new("LEMON".to_string());
        let cipher = v.to_ciphertext(&"ATTACKATDAWN".as_bytes().to_vec());
        assert_eq!(cipher, "LXFOPVEFRNHR".as_bytes().to_vec());
        let plain = v.to_plaintext(&cipher);
        assert_eq!(plain, "ATTACKATDAWN".as_bytes().to_vec());
    }

    #[test]
    fn mixed_case_and_spaces() {
        let mut v = Vigenere::new("LeMon".to_string());
        let cipher = v.to_ciphertext(&"Attack at Dawn!".as_bytes().to_vec());
        // Verificar round-trip apenas
        let mut v2 = Vigenere::new("LEMON".to_string());
        let plain = v2.to_plaintext(&cipher);
        assert_eq!(plain, "Attack at Dawn!".as_bytes().to_vec());
    }
}
