extern crate caesarcy;

use caesarcy::{encipher, decipher};


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ciphered_text_can_be_deciphered() {
        assert_eq!("Toto", decipher(13, &encipher(13, "Toto")));
    }
    #[test]
    fn ciphered_text_is_different_with_regards_to_offset() {
        assert_ne!(encipher(13, "Toto"), encipher(14, "Toto"));
    }
    #[test]
    fn ciphered_text_can_contains_unknown_chars() {
        assert_eq!("Foo:bar.", decipher(13, &encipher(13, "Foo:bar.")));
    }
}
