from i_encryption import I_Encryption


class Cezer_Cypher(I_Encryption):

    def __init__(self) -> None:
        super().__init__()

    
    def encrypt(self, plain_text: str) -> str:
        cypher_text: str = ""

        for x in plain_text:

            if not x.isalpha():
                continue

            new_char = (ord(x.upper()) + 3)

            if new_char > ord("Z"):
                new_char -= 26

            cypher_text += chr(new_char)

        return cypher_text

    
    def decrypt(self, cypher_text: str) -> str:
        plain_text: str = ""

        for x in cypher_text:
            new_char = (ord(x.upper()) - 3)

            if(new_char < ord("A")):
                new_char += 26

            plain_text += chr(new_char)

        return plain_text