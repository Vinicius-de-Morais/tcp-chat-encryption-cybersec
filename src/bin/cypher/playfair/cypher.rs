use std::{collections::HashSet, fmt};

use crate::cypher::{playfair::matrix::KeyMatrix, Cypher};

/// A group of two successive letters
type Digraph = [char; 2];

struct Playfair {
    matrix: KeyMatrix,
}

impl Playfair {
    pub fn new(key: String) -> Self {
        return Playfair {
            matrix: KeyMatrix::new(key),
        };
    }

    fn explode_input(source: &String) -> Vec<Digraph> {
        let valid_chars: Vec<char> = source
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .filter(|c| return c >= &'A' && c <= &'Z') // só permitir chars A-Z
            .map(|c| if c == 'I' { 'J' } else { c }) // substituir 'i' por 'j'
            .collect();

        for i in 0..source.len() {}

        todo!()
    }
}

impl Cypher for Playfair {
    fn to_cyphertext(&mut self, plaintext: String) -> String {
        //   let digraphs = Playfair::explode(&plaintext);
        todo!()
    }

    fn to_plaintext(&mut self, cyphertext: String) -> String {
        //    let digraphs = Playfair::explode(&plaintext);
        todo!()
    }
}
