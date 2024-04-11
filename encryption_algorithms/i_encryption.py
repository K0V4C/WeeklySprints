# Base class for all Encyptions

class I_Encryption():
    def __init__(self) -> None:
        pass

    def encrypt(self, plain_text: str) -> str:
        return ""

    def decrypt(self, cypher_text: str) -> str:
        return ""