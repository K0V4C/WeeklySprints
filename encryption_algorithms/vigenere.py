from i_encryption import I_Encryption

class Vigenere(I_Encryption):

    def __init__(self, key="", autokey=False) -> None:
        self.key = key.upper()
        self.autokey = autokey

        super().__init__()

    def encrypt(self, plain_text: str) -> str:

        plain_text = plain_text.upper()
        plain_text = plain_text.replace(" ", "")

        new_key = self.key[:]
        
        if self.autokey == False:
            while(len(new_key) < len(plain_text)):
                new_key += self.key[:]
        else:
            new_key += plain_text[:]

        self.key = new_key

        cypher_text = ""

        idx: int = 0
        while(idx < len(plain_text)):

            ch1 = plain_text[idx]
            ch2 = self.key[idx]

            

            new_ch = chr((ord(ch1) + ord(ch2) - 2*ord("A")) % 26 + ord("A"))


            # print(f'{ch1}       {ch2}        {new_ch}            {ord(ch1) + ord(ch2) - ord("A")}')
            cypher_text += new_ch

            idx += 1


        return cypher_text
    
    def decrypt(self, cypher_text: str) -> str:

        cypher_text = cypher_text.replace(" ", "").upper()


        plain_text = ""
        idx: int = 0
        while(idx < len(cypher_text)):

            ch1 = cypher_text[idx]
            ch2 = self.key[idx]

            new_ch = chr((ord(ch1) - ord(ch2)) % 26 + ord("A"))

            # print(f'{ch1}       {ch2}        {new_ch}            {ord(ch1) + ord(ch2) - ord("A")}')
            plain_text += new_ch

            idx += 1


        return plain_text



        return ""