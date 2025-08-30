![Logo](./meta/logo.png)

Esse é um projeto de cyberseguraça para exercitar o conhecimento de cifras.

# VERY GOOD CHAT PROJECT

## Resolutions

1. O username vai ser gerado automaticamente pelo servidor.
2. Seleção de como o usuário vai encriptar a mensagem. Assim como a chave.
3. Para decriptar uma mensagem o usuário clica na mensagem e digita a palavra passe.
3.1. O client vai tentar fazer bruteforce pra quebrar a chave.

## Cifras

- [x] Crifra de César
- [x] Substituição Monoalfabética
- [x] Cifra de Playfair
- [x] Cifra de Vigenère

## Regras e instruções

1.  Conexão ao servidor:

        O usuário deve digitar o IP do servidor ao qual deseja se conectar.

2.  Escolha da cifra:

        Após a conexão, o programa deve exibir um menu para o usuário escolher qual cifra será usada para criptografar as mensagens.

3.  Definição da chave:

        O usuário deve inserir a chave secreta previamente combinada com o parceiro (a chave varia conforme o tipo de cifra).

4.  Envio e recebimento de mensagens:

        Antes de enviar, a mensagem deve ser criptografada.

        Ao chegar no destinatário, deve ser descriptografada automaticamente.

5.  Função de verificação no servidor:

        O servidor deve imprimir no console as mensagens para facilitar a conferência e depuração.

6.  Ambiente de execução:

        A apresentação será feita em rede local no laboratório do SENAI.

7.  Restrições:

        Não é permitido usar funções prontas de bibliotecas para criptografia.
        Todas as funções de criptografia e descriptografia devem ser implementadas manualmente.

## Validações RC4

```
TESTE 1:
	Entrada:
		Texto Plano = "Cybersecurity melhor disciplina do curso.";

		Texto Plano ASCII = [67 121 98 101 114 115 101 99 117 114 105 116 121 32 109 101 108 104 111 114 32 100 105 115 99 105 112 108 105 110 97 32 100 111 32 99 117 114 115 111 46];
		
		Chave= D&Ot)[YW

		Chave ASCII = [68, 38, 79, 116, 41, 91, 89, 87];

		Texto Cript. = [214 32 110 109 116 251 159 133 226 76 193 253 168 73 65 197 82 72 93 68 250 55 28 202 59 77 186 27 97 24 48 54 106 38 82 214 222 20 20 13 251]

TESTE 2:
	Entrada:
		Texto Plano = "Cybersecurity melhor disciplina do curso.";

		Texto Plano ASCII = [67 121 98 101 114 115 101 99 117 114 105 116 121 32 109 101 108 104 111 114 32 100 105 115 99 105 112 108 105 110 97 32 100 111 32 99 117 114 115 111 46];

		Chave= $@C*9)6C{4^dXNw>H#W,be/\'L2pM8r;JY?x}B]@A`T!q?iO`=n.Lgm(3z8@S[u]dY1k|%RI!MP-(FtZl&^3:jnK<TG6[5Jw}

		Chave ASCII = [36 64 67 42 57 41 54 67 123 52 94 100 88 78 119 62 72 35 87 44 98 101 47 92 39 76 50 112 77 56 114 59 74 89 63 120 125 66 93 64 65 96 84 33 113 63 105 79 96 61 110 46 76 103 109 40 51 122 56 64 83 91 117 93 100 89 49 107 124 37 82 73 33 77 80 45 40 70 116 90 108 38 94 51 58 106 110 75 60 84 71 54 91 53 74 119 125];

		Texto Cript. = [84 179 117 15 203 82 18 217 141 197 213 126 47 255 83 83 99 47 120 247 192 203 33 247 220 192 213 82 241 248 166 142 129 105 50 227 178 74 181 144 94];


T3: 

Entrada:
		Texto Plano = "Cybersecurity melhor disciplina do curso.";

		Texto Plano ASCII = [67 121 98 101 114 115 101 99 117 114 105 116 121 32 109 101 108 104 111 114 32 100 105 115 99 105 112 108 105 110 97 32 100 111 32 99 117 114 115 111 46];

		Chave= !M|7s]u^{DFj^?8+fL:0Z!*%1P_3B}9m~V0@H^Qf7y&Z4Wb>kS^T<d.$.pL@R|g)x)-6(E&h%T-}(W%z{U9mZz8~m8BfP!c&@k7I\5I~T_vD!4A>|oO[}3*T|$?e~0]V5&y@r1X2k+@T]j?|2|Q%}R,D)Up\8gM;W}|7eNFk^t.h/j;6#y-!t5)\^LJ[7S<4A,f$Ks1|&sX!w*G(Z@i>jE>6~]oA5]k'.:o=7n9h)$J_!aB{N-Jb1M}NzD\*h


		Chave ASCII = [33 77 124 55 115 93 117 94 123 68 70 106 94 63 56 43 102 76 58 48 90 33 42 37 49 80 95 51 66 125 57 109 126 86 48 64 72 94 81 102 55 121 38 90 52 87 98 62 107 83 94 84 60 100 46 36 46 112 76 64 82 124 103 41 120 41 45 54 40 69 38 104 37 84 45 125 40 87 37 122 123 85 57 109 90 122 56 126 109 56 66 102 80 33 99 38 64 107 55 73 92 53 73 126 84 95 118 68 33 52 65 62 124 111 79 91 125 51 42 84 124 36 63 101 126 48 93 86 53 38 121 64 114 49 88 50 107 43 64 84 93 106 63 124 50 124 81 37 125 82 44 68 41 85 112 92 56 103 77 59 87 125 124 55 101 78 70 107 94 116 46 104 47 106 59 54 35 121 45 33 116 53 41 92 94 76 74 91 55 83 60 52 65 44 102 36 75 115 49 124 38 115 88 33 119 42 71 40 90 64 105 62 106 69 62 54 126 93 111 65 53 93 107 39 46 58 111 61 55 110 57 104 41 36 74 95 33 97 66 123 78 45 74 98 49 77 125 78 122 68 92 42 104];

		Texto Cript. = [192 115 138 155 179 72 115 33 116 105 228 122 36 92 74 122 123 245 202 209 214 199 4 191 90 96 21 15 190 222 47 58 192 192 43 10 166 63 58 96 230];


```