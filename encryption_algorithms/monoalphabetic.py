from i_encryption import I_Encryption
class Monoalphabetic(I_Encryption):

    def __init__(self, permutation_table: list[str]) -> None:

        self.permutation_table = permutation_table

        super().__init__()

    def encrypt(self, plain_text: str) -> str:
        cypher_text = ""
        for x in plain_text:

            if not x.isalpha():
                continue

            pos = ord(x.upper()) - ord("A")
            cypher_text += self.permutation_table[pos]

        return cypher_text


    def decrypt(self, cypher_text: str) -> str:
        plain_text: str = ""
        for x in cypher_text:
            char: str = x.upper()
            plain_text += chr(ord("A") + self.permutation_table.index(char))

        return plain_text