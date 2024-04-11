from i_encryption import I_Encryption
import re

class PlayFair(I_Encryption):

    alphabet_table = [  "A", "B", "C", "D"  , "E",
                        "F", "G", "H", "?","K",
                        "L", "M", "N", "O"  ,"P",
                        "Q", "R", "S", "T"  ,"U",
                        "V", "W", "X", "Y"  ,"Z"]

    def __init__(self, key: str) -> None:

        self.key = key
        self.filler = "X"

        self.cypher: list[list[str]] = [[None for x in range(5)] for x in range(5)]

        self.generate_cypher()

        print(self.cypher)

        super().__init__()

    def generate_cypher(self):
        # construct the cypher

        self.key = self.key.upper()
        self.key = self.key.replace("I", "?")
        self.key = self.key.replace("J", "?")

        unique_chars = set(self.key)
        alphabet: list[str] = self.alphabet_table[:]

        idx: int = 0

        for x in self.key:
            if x in unique_chars:

                unique_chars.remove(x)
                alphabet.remove(x)
                self.cypher[idx // 5][idx % 5] = x
                idx += 1

        for x in alphabet:
            self.cypher[idx // 5][idx % 5] = x
            idx += 1

    def generate_digrams(self, text: str) -> list[str]:

        digrams: list[str] = list()
        idx: int = 0

        while idx < len(text):

            ch1 = text[idx]
            ch2 = None


            if idx + 1 < len(text):
                ch2 = text[idx+1]

            if ch1 == ch2:
                ch2 = self.filler
                idx += 1
            else:
                idx += 2

            if ch2 == None:
                ch2 = self.filler

            digrams.append(""+ch1.upper()+ch2.upper())

        
        return digrams

    def find_x_y(self, chr: str) -> tuple[int,int]:

        if chr == "I" or chr == "J":
            chr = "?"
        for i in range(len(self.cypher)):
            for j in range(len(self.cypher[0])):
                if chr == self.cypher[i][j]:
                    return (i,j)
                
        return (-1,-1)


    def encrypt(self, plain_text: str) -> str:

        plain_text = plain_text.upper()
        plain_text = plain_text.replace("I", "?")
        plain_text = plain_text.replace("J", "?")
        plain_text = plain_text.replace(" ", "")

        digrams: list[str] = self.generate_digrams(plain_text)
        encypted_digrams = list()

        # print(digrams)

        for digram in digrams:

            ch1 = digram[0]
            ch2 = digram[1]

            # (int,int)
            pos1 = self.find_x_y(ch1)
            pos2 = self.find_x_y(ch2)

            # same row
            if pos1[0] == pos2[0]:
                
                new_ch1: str = self.cypher[(pos1[0] + 1) % 5][pos1[1]]
                new_ch2: str = self.cypher[(pos2[0] + 1) % 5][pos2[1]]

                encypted_digrams.append(""+new_ch1+new_ch2)

            # same column
            elif pos1[1] == pos2[1]:
                new_ch1: str = self.cypher[pos1[0]][(pos1[1] + 1) % 5]
                new_ch2: str = self.cypher[pos2[0]][(pos2[1] + 1) % 5]

                encypted_digrams.append(""+new_ch1+new_ch2)
            # box
            else:

                new_ch1: str = self.cypher[pos1[0]][(pos2[1]) % 5]
                new_ch2: str = self.cypher[pos2[0]][(pos1[1]) % 5]

                encypted_digrams.append(""+new_ch1+new_ch2)
                
        
        cypher_text = ""

        for x in encypted_digrams:
            cypher_text += x

        return cypher_text
    
    def decrypt(self, cypher_text: str) -> str:

        digrams: list[str] = self.generate_digrams(cypher_text)
        decrypted_digrams = list()

        for digram in digrams:

            ch1 = digram[0]
            ch2 = digram[1]

            # (int,int)
            pos1 = self.find_x_y(ch1)
            pos2 = self.find_x_y(ch2)

            # same row
            if pos1[0] == pos2[0]:
                
                new_ch1: str = self.cypher[(pos1[0] - 1) % 5][pos1[1]]
                new_ch2: str = self.cypher[(pos2[0] - 1) % 5][pos2[1]]

                decrypted_digrams.append(""+new_ch1+new_ch2)

            # same column
            elif pos1[1] == pos2[1]:
                new_ch1: str = self.cypher[pos1[0]][(pos1[1] - 1) % 5]
                new_ch2: str = self.cypher[pos2[0]][(pos2[1] - 1) % 5]

                decrypted_digrams.append(""+new_ch1+new_ch2)
            # box
            else:

                new_ch1: str = self.cypher[pos1[0]][(pos2[1]) % 5]
                new_ch2: str = self.cypher[pos2[0]][(pos1[1]) % 5]

                decrypted_digrams.append(""+new_ch1+new_ch2)
                
        
        plain_text = ""

        for x in decrypted_digrams:
            plain_text += x

        return plain_text