use crate::ciphers::Cipher;

pub const ARRAY_SIZE : usize = 256;

pub struct Rc4 {
    passkey: String,
    key: Vec<u8>,
    s: Vec<u8>,
    i: usize,
    j: usize,
    decripted: Vec<u8>
}

impl Rc4 {
    pub fn new(passkey: String) -> Rc4{
        let mut s: Vec<u8> = (0..ARRAY_SIZE).map(|x| x as u8).collect();
        let key: Vec<u8> = passkey.as_bytes().to_vec();
        let mut swap_pos = 0;

        // Prevent division by zero and unwrap panic if key is empty
        if key.is_empty() {
            panic!("RC4 key cannot be empty");
        }

        for pos in 0..ARRAY_SIZE {
            let letterpos = pos % key.len();

            let actual_item_value = s.get(pos).unwrap().clone() as usize;
            let actual_key_letter = key.get(letterpos).unwrap().clone() as usize;

            swap_pos = (swap_pos + actual_item_value + actual_key_letter) % ARRAY_SIZE;

            s.swap(pos, swap_pos);
        }

        Rc4{
            passkey,
            key,
            s,
            i: 0,
            j: 0,
            decripted: Vec::new()
        }
    }

    pub fn process(&mut self, plaintext: String) -> Vec<u8>{
        let input = plaintext.as_bytes();

        let mut output = Vec::with_capacity(input.len());

        for &byte in input {
            
            // processo de "resgatar" os indexes.
            self.i = (self.i + 1) % ARRAY_SIZE;
            self.j = (self.j + self.s[self.i] as usize) % ARRAY_SIZE;

            self.s.swap(self.i, self.j);
            let t = (self.s[self.i] as usize + self.s[self.j] as usize) % ARRAY_SIZE;

            // resgatar caractere K
            let k = self.s[t];
            output.push(byte ^ k);
        }

        self.decripted = output.clone();

        // Imprime o output em hexadecimal para facilitar a validação
        // print!("[ ");
        // for byte in &output {
        //     print!("{:02X} ", byte);
        // }
        // println!("]");

        output
    }

    fn swap(mut vec: Vec<u8>, pos_a: usize, pos_b: usize) {
        vec.swap(pos_a, pos_b)
    }
}

impl Cipher for Rc4 {
    fn to_ciphertext(&mut self, plaintext: &String) -> String {
        //String::from_utf8(self.process(plaintext.to_string())).unwrap()

        let ascii_string: Vec<String> = self.process(plaintext.to_string())
            .iter()
            .map(|&x| x.to_string())
            .collect();

        ascii_string.join(" ")
    }

    fn to_plaintext(&mut self, ciphertext: &String) -> String {
        //String::from_utf8(self.process(ciphertext.to_string())).unwrap()

        let ascii_string: Vec<String> = self.process(ciphertext.to_string())
            .iter()
            .map(|&x| (x as usize).to_string())
            .collect();

        ascii_string.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc4_case1() {
        let key = "D&Ot)[YW";
        let plaintext = "Cybersecurity melhor disciplina do curso.";
        let expected_cipher = [
            214, 32, 110, 109, 116, 251, 159, 133, 226, 76, 193, 253, 168, 73, 65, 197, 82, 72, 93, 68, 250, 55, 28, 202, 59, 77, 186, 27, 97, 24, 48, 54, 106, 38, 82, 214, 222, 20, 20, 13, 251
        ];
        let mut rc4 = Rc4::new(key.to_string());
        let cipher = rc4.process(plaintext.to_owned());
        assert_eq!(cipher, expected_cipher);
    }

    #[test]
    fn test_rc4_case2() {
        let key = "$@C*9)6C{4^dXNw>H#W,be/\\'L2pM8r;JY?x}B]@A`T!q?iO`=n.Lgm(3z8@S[u]dY1k|%RI!MP-(FtZl&^3:jnK<TG6[5Jw}";
        let plaintext = "Cybersecurity melhor disciplina do curso.";
        let expected_cipher = [
            84, 179, 117, 15, 203, 82, 18, 217, 141, 197, 213, 126, 47, 255, 83, 83, 99, 47, 120, 247, 192, 203, 33, 247, 220, 192, 213, 82, 241, 248, 166, 142, 129, 105, 50, 227, 178, 74, 181, 144, 94
        ];
        let mut rc4 = Rc4::new(key.to_string());
        let cipher = rc4.process(plaintext.to_owned());
        assert_eq!(cipher, expected_cipher);
    }

    #[test]
    fn test_rc4_case3() {
        let key = "!M|7s]u^{DFj^?8+fL:0Z!*%1P_3B}9m~V0@H^Qf7y&Z4Wb>kS^T<d.$.pL@R|g)x)-6(E&h%T-}(W%z{U9mZz8~m8BfP!c&@k7I\\5I~T_vD!4A>|oO[}3*T|$?e~0]V5&y@r1X2k+@T]j?|2|Q%}R,D)Up\\8gM;W}|7eNFk^t.h/j;6#y-!t5)\\^LJ[7S<4A,f$Ks1|&sX!w*G(Z@i>jE>6~]oA5]k'.:o=7n9h)$J_!aB{N-Jb1M}NzD\\*h";
        let plaintext = "Cybersecurity melhor disciplina do curso.";
        let expected_cipher = [
            192, 115, 138, 155, 179, 72, 115, 33, 116, 105, 228, 122, 36, 92, 74, 122, 123, 245, 202, 209, 214, 199, 4, 191, 90, 96, 21, 15, 190, 222, 47, 58, 192, 192, 43, 10, 166, 63, 58, 96, 230
        ];
        let mut rc4 = Rc4::new(key.to_string());
        let cipher = rc4.process(plaintext.to_owned());
        assert_eq!(cipher, expected_cipher);
    }
}