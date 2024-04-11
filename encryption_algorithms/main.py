from i_encryption import I_Encryption
from cezer_cypher import Cezer_Cypher
from monoalphabetic import Monoalphabetic
from playfair import PlayFair
from vigenere import Vigenere


def test(msg: str, engine: I_Encryption):
    cypher_text = engine.encrypt(msg)
    print(cypher_text)

    plain_text = engine.decrypt(cypher_text)
    print(plain_text)

def main() -> None:
    engine = Cezer_Cypher()

    alphabet_table = [    "B", "A", "C", "D", "E",
                          "F", "G", "H", "I", "J",
                          "K", "L", "M", "N", "O",
                          "P", "Q", "R", "S", "T",
                          "U", "V", "W", "X", "Y",
                          "Z"]
    engine = Monoalphabetic(alphabet_table) # TODO: add Gauss

    key = "SUNCCCCE"
    engine = PlayFair(key=key)

    # key = "Kljuc"
    # engine = Vigenere(key=key, autokey=True)


    test("SA AVALOM PLAVOM U DALJINI KAO BREG", engine=engine)

if __name__ == "__main__":
    main()