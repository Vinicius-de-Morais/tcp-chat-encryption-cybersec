use crate::ciphers::{
    playfair::matrix::{KeyMatrix, Pos},
    Cipher,
};

/// A group of two successive letters
type Digraph = [char; 2];

pub struct Playfair {
    matrix: KeyMatrix,
}

impl Playfair {
    pub fn new(key: String) -> Self {
        return Playfair {
            matrix: KeyMatrix::new(key),
        };
    }

    fn prepare_plaintext_input(source: &Vec<u8>) -> Vec<Digraph> {
        let mut filtered_chars: Vec<char> = source
            .into_iter()
            .map(|c| char::from(*c))
            .map(|c| c.to_ascii_uppercase())
            .filter(|c| return c >= &'A' && c <= &'Z') // só permitir chars A-Z
            .map(|c| if c == 'I' { 'J' } else { c }) // substituir 'i' por 'j'
            .collect();

        if filtered_chars.len() % 2 != 0 {
            filtered_chars.push('X'); // fazer padding com letra X
        }

        // separar letras iguais por X
        // ex. "CARRO" -> "CARXRO"
        let mut separated = vec![];
        for i in 0..filtered_chars.len() {
            let current = filtered_chars[i];
            separated.push(current);
            if let Some(next) = filtered_chars.get(i + 1) {
                if *next == current {
                    separated.push('X');
                }
            }
        }

        // separar em digrafos
        return separated
            .chunks_exact(2)
            .map(|chars| [chars[0], chars[1]] as Digraph)
            .collect();
    }

    fn prepare_cipher_input(source: &Vec<u8>) -> Vec<Digraph> {
        assert!(
            source.len() % 2 == 0,
            "texto cifrado não é múltiplo de 2, entrada inválida"
        );

        let filtered_chars: Vec<char> = source
            .into_iter()
            .map(|c| char::from(*c))
            .map(|c| c.to_ascii_uppercase())
            .filter(|c| return c >= &'A' && c <= &'Z') // só permitir chars A-Z
            .map(|c| if c == 'I' { 'J' } else { c }) // substituir 'i' por 'j'
            .collect();

        // separar em digrafos
        return filtered_chars
            .chunks_exact(2)
            .map(|chars| [chars[0], chars[1]] as Digraph)
            .collect();
    }
}

impl Cipher for Playfair {
    fn to_ciphertext(&mut self, plaintext: &Vec<u8>) -> Vec<u8> {
        let input = Playfair::prepare_plaintext_input(&plaintext);
        let mut output: Vec<char> = vec![];

        for digraph in input {
            let pos_a = self.matrix.get_position(digraph[0]);
            let pos_b = self.matrix.get_position(digraph[1]);

            if pos_a.column == pos_b.column {
                let n_a = self.matrix.get_char(&Pos {
                    row: (pos_a.row + 1) % 5,
                    column: pos_a.column,
                });

                let n_b = self.matrix.get_char(&Pos {
                    row: (pos_b.row + 1) % 5,
                    column: pos_b.column,
                });

                output.push(n_a);
                output.push(n_b);
            } else if pos_a.row == pos_b.row {
                let n_a = self.matrix.get_char(&Pos {
                    row: pos_a.row,
                    column: (pos_a.column + 1) % 5,
                });

                let n_b = self.matrix.get_char(&Pos {
                    row: pos_b.row,
                    column: (pos_b.column + 1) % 5,
                });

                output.push(n_a);
                output.push(n_b);
            } else {
                let n_a = self.matrix.get_char(&Pos {
                    row: pos_a.row,
                    column: pos_b.column,
                });

                let n_b = self.matrix.get_char(&Pos {
                    row: pos_b.row,
                    column: pos_a.column,
                });

                output.push(n_a);
                output.push(n_b);
            }
        }

        String::from_iter(output.iter()).as_bytes().to_vec()
    }

    fn to_plaintext(&mut self, ciphertext: &Vec<u8>) -> Vec<u8> {
        let input = Playfair::prepare_cipher_input(&ciphertext);
        let mut output: Vec<char> = vec![];

        for digraph in input {
            let pos_a = self.matrix.get_position(digraph[0]);
            let pos_b = self.matrix.get_position(digraph[1]);

            if pos_a.column == pos_b.column {
                let n_a = self.matrix.get_char(&Pos {
                    row: (pos_a.row + 4) % 5,
                    column: pos_a.column,
                });

                let n_b = self.matrix.get_char(&Pos {
                    row: (pos_b.row + 4) % 5,
                    column: pos_b.column,
                });

                output.push(n_a);
                output.push(n_b);
            } else if pos_a.row == pos_b.row {
                let n_a = self.matrix.get_char(&Pos {
                    row: pos_a.row,
                    column: (pos_a.column + 4) % 5,
                });

                let n_b = self.matrix.get_char(&Pos {
                    row: pos_b.row,
                    column: (pos_b.column + 4) % 5,
                });

                output.push(n_a);
                output.push(n_b);
            } else {
                let n_a = self.matrix.get_char(&Pos {
                    row: pos_a.row,
                    column: pos_b.column,
                });

                let n_b = self.matrix.get_char(&Pos {
                    row: pos_b.row,
                    column: pos_a.column,
                });

                output.push(n_a);
                output.push(n_b);
            }
        }

        String::from_iter(output.iter()).as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_explode() {
        let digraphs =
            Playfair::prepare_plaintext_input(&"AULADESEGURANCADAINFORMACAO".as_bytes().to_vec());

        assert_eq!(
            digraphs,
            [
                ['A', 'U'],
                ['L', 'A'],
                ['D', 'E'],
                ['S', 'E'],
                ['G', 'U'],
                ['R', 'A'],
                ['N', 'C'],
                ['A', 'D'],
                ['A', 'J'], // i substituido por j
                ['N', 'F'],
                ['O', 'R'],
                ['M', 'A'],
                ['C', 'A'],
                ['O', 'X'] // padding X
            ]
        );

        assert_eq!(
            Playfair::prepare_plaintext_input(&"SECRET MESSAGE".as_bytes().to_vec()),
            [
                ['S', 'E'],
                ['C', 'R'],
                ['E', 'T'],
                ['M', 'E'],
                ['S', 'X'], // "X" inserido entre SS
                ['S', 'A'],
                ['G', 'E']
            ]
        );
    }

    #[test]
    fn cipher_and_back() {
        let mut cipher = Playfair::new("Aula de Seguranca da Informacao".to_string());
        let ciphered = cipher.to_ciphertext(
            &"Bomba Nuclear no Rio de Janeiro  xxxxdoisxxxmilxxxvintexxxseis"
                .as_bytes()
                .to_vec(), // Playfair não suporta números
        );

        println!("Matrix: {}", cipher.matrix);
        println!("Ciphered: {:?}", ciphered);
        let deciphered = cipher.to_plaintext(&ciphered);
        println!("Deciphered: {:?}", deciphered);

        assert_eq!(
            deciphered,
            "BOMBANUCLEARNORJODEJANEJROXXXXXXXDOJSXXXXXMJLXXXXXVJNTEXXXXXSEJS"
                .as_bytes()
                .to_vec()
        );
    }
}
