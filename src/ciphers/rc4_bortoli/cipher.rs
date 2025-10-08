use crate::ciphers::Cipher;

pub struct Rc4Bortoli {
    original_key: String,
    S: [u8; 256],
    i: usize,
    j: usize,
}

impl Rc4Bortoli {
    pub fn new(key: String) -> Self {
        let mut rc4 = Rc4Bortoli {
            original_key: key,
            S: [0u8; 256],
            i: 0,
            j: 0,
        };

        rc4.reset();

        rc4
    }

    fn reset(&mut self) {
        // um vetor S, de tamanho 256, é inicializado de forma que S[0] = 0, S[1] = 1, ..., S[255] = 255
        for i in 0..256 {
            self.S[i] = i as u8;
        }

        // um vetor T, de 256 bytes, é inicializado com o conteúdo da chave repetido
        let chave = self.original_key.as_bytes();
        let mut vT = [0u8; 256];
        for i in 0..256 {
            vT[i] = chave[i % chave.len()];
        }

        // em seguida, usamos T para produzir uma permutacao inicial de S.
        // isso envolve começar com S[0] e ir até S[255], e, ao longo do caminho, trocar S[i] por outro byte em S, cujo indice sera computado em j.
        let mut j = 0;
        for i in 0..256 {
            j = (j + (self.S[i] as usize) + (vT[i] as usize)) % 256;

            //fazer swap(S[i], S[j])
            let tmp = self.S[i];
            self.S[i] = self.S[j];
            self.S[j] = tmp;
        }
    }

    fn process_single(&mut self, input: u8) -> u8 {
        // aqui é simples:
        // 1. geramos um byte de keystream
        // 2. byte_cifrado = xor(byte_plano, byte_keystream)

        self.i = (self.i + 1) % 256;
        self.j = (self.j + (self.S[self.i] as usize)) % 256;
        //swap(S[i], S[j])
        let tmp = self.S[self.i];
        self.S[self.i] = self.S[self.j];
        self.S[self.j] = tmp;

        let t = ((self.S[self.i] as usize) + (self.S[self.j] as usize)) % 256;
        let byte_keystream = self.S[t];

        let byte_cifrado = input ^ byte_keystream;

        byte_cifrado
    }
}

impl Cipher for Rc4Bortoli {
    fn to_ciphertext(&mut self, plaintext: &String) -> String {
        let ciphered = plaintext
            .bytes()
            .map(|byte| self.process_single(byte))
            .collect::<Vec<u8>>();

        String::from_utf8_lossy(&ciphered).to_string()
    }

    fn to_plaintext(&mut self, ciphertext: &String) -> String {
        let ciphered = ciphertext
            .bytes()
            .map(|byte| self.process_single(byte))
            .collect::<Vec<u8>>();

        String::from_utf8_lossy(&ciphered).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc4bortoli_case1() {
        let key = "D&Ot)[YW";
        let plaintext = "Cybersecurity melhor disciplina do curso.";
        let expected_cipher = [
            214, 32, 110, 109, 116, 251, 159, 133, 226, 76, 193, 253, 168, 73, 65, 197, 82, 72, 93,
            68, 250, 55, 28, 202, 59, 77, 186, 27, 97, 24, 48, 54, 106, 38, 82, 214, 222, 20, 20,
            13, 251,
        ];
        let mut rc4 = Rc4Bortoli::new(key.to_string());
        let ciphered = plaintext
            .as_bytes()
            .iter()
            .map(|b| rc4.process_single(*b))
            .collect::<Vec<u8>>();
        assert_eq!(ciphered, expected_cipher);
    }

    #[test]
    fn test_rc4bortoli_case2() {
        let key = "$@C*9)6C{4^dXNw>H#W,be/\\'L2pM8r;JY?x}B]@A`T!q?iO`=n.Lgm(3z8@S[u]dY1k|%RI!MP-(FtZl&^3:jnK<TG6[5Jw}";
        let plaintext = "Cybersecurity melhor disciplina do curso.";
        let expected_cipher = [
            84, 179, 117, 15, 203, 82, 18, 217, 141, 197, 213, 126, 47, 255, 83, 83, 99, 47, 120,
            247, 192, 203, 33, 247, 220, 192, 213, 82, 241, 248, 166, 142, 129, 105, 50, 227, 178,
            74, 181, 144, 94,
        ];
        let mut rc4 = Rc4Bortoli::new(key.to_string());
        let ciphered = plaintext
            .as_bytes()
            .iter()
            .map(|b| rc4.process_single(*b))
            .collect::<Vec<u8>>();
        assert_eq!(ciphered, expected_cipher);
    }

    #[test]
    fn test_rc4bortoli_case3() {
        let key = "!M|7s]u^{DFj^?8+fL:0Z!*%1P_3B}9m~V0@H^Qf7y&Z4Wb>kS^T<d.$.pL@R|g)x)-6(E&h%T-}(W%z{U9mZz8~m8BfP!c&@k7I\\5I~T_vD!4A>|oO[}3*T|$?e~0]V5&y@r1X2k+@T]j?|2|Q%}R,D)Up\\8gM;W}|7eNFk^t.h/j;6#y-!t5)\\^LJ[7S<4A,f$Ks1|&sX!w*G(Z@i>jE>6~]oA5]k'.:o=7n9h)$J_!aB{N-Jb1M}NzD\\*h";
        let plaintext = "Cybersecurity melhor disciplina do curso.";
        let expected_cipher = [
            192, 115, 138, 155, 179, 72, 115, 33, 116, 105, 228, 122, 36, 92, 74, 122, 123, 245,
            202, 209, 214, 199, 4, 191, 90, 96, 21, 15, 190, 222, 47, 58, 192, 192, 43, 10, 166,
            63, 58, 96, 230,
        ];
        let mut rc4 = Rc4Bortoli::new(key.to_string());
        let ciphered = plaintext
            .as_bytes()
            .iter()
            .map(|b| rc4.process_single(*b))
            .collect::<Vec<u8>>();
        assert_eq!(ciphered, expected_cipher);
    }
}
