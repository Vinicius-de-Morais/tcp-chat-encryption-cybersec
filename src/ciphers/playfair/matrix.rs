use std::fmt;

use crate::ciphers::playfair::helper::Unique;

pub struct KeyMatrix {
    original_key: String,
    contents: [char; 25],
}

impl KeyMatrix {
    pub fn new(key: String) -> Self {
        let mut key_chars: Vec<char> = key
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .filter(|c| return c >= &'A' && c <= &'Z') // só permitir chars A-Z
            .map(|c| if c == 'I' { 'J' } else { c }) // substituir 'i' por 'j'
            .collect();

        key_chars.unique(); // remover duplicatas da chave

        let alphabet: Vec<char> = "ABCDEFGHJKLMNOPQRSTUVWXYZ" // alfabeto com 'i' trocado por 'j'
            .chars()
            .filter(|l| !key_chars.contains(l)) // alfabeto com as letras da chave removidas
            .collect();

        let mut contents = ['\0'; 25];

        let mut mpos = 0;
        for key_char in key_chars {
            contents[mpos] = key_char;
            mpos = mpos + 1;
        }

        for alphabet_char in alphabet {
            contents[mpos] = alphabet_char;
            mpos = mpos + 1;
        }

        return KeyMatrix {
            original_key: key,
            contents,
        };
    }

    pub fn get_position(&self, mut ch: char) -> Pos {
        ch = ch.to_ascii_uppercase();

        assert!(ch >= 'A' && ch <= 'Z', "Caractere inválido!");

        let idx = self.contents.iter().position(|&r| r == ch).unwrap();

        let row = idx / 5;
        let column = idx % 5;

        return Pos { row, column };
    }

    pub fn get_char(&self, position: &Pos) -> char {
        assert!(position.row < 5);
        assert!(position.column < 5);

        return self.contents[position.row * 5 + position.column];
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos {
    pub row: usize,
    pub column: usize,
}

impl fmt::Display for KeyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Chave original: {}", self.original_key)?;
        for row in 0..5 {
            for col in 0..5 {
                let index = row * 5 + col;
                write!(f, "{} ", self.contents[index])?;
            }
            writeln!(f)?; // proxima linha
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_matrix_igual_ao_pdf() {
        let matrix = KeyMatrix::new("INFORMATICA".to_string());
        eprintln!("{}", matrix);
        assert_eq!(
            matrix.contents.to_vec(),
            "JNFORMATCBDEGHKLPQSUVWXYZ".chars().collect::<Vec<char>>() // matriz-chave dada no PDF da atividade
        )
    }

    #[test]
    fn key_matrix_filtros_caracteres() {
        let matrix = KeyMatrix::new("infoR175**'':::  4234 23MATICAAAAAAAAAAAAAA".to_string());
        eprintln!("{}", matrix);
        assert_eq!(
            matrix.contents.to_vec(),
            "JNFORMATCBDEGHKLPQSUVWXYZ".chars().collect::<Vec<char>>() // matriz-chave dada no PDF da atividade
        )
    }
}
