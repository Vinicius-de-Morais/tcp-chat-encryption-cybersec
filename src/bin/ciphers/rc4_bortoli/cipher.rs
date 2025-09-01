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
