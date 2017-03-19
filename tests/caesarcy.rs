extern crate caesarcy;

#[cfg(test)]
mod tests {
    #[test]
    fn ciphered_text_can_be_deciphered() {
        assert_eq!("Toto", caesarcy::decipher(13, caesarcy::encipher(13, "Toto")));
    }
    #[test]
    fn ciphered_text_is_different_with_regards_to_offset() {
        assert_ne!(caesarcy::encipher(13, "Toto"), caesarcy::encipher(14, "Toto"));
    }
    #[test]
    fn ciphered_text_can_contains_unknown_chars() {
        assert_eq!("Foo:bar.", caesarcy::decipher(13, caesarcy::encipher(13, "Foo:bar.")));
    }
}
