//! Code Originally found https://github.com/TheAlgorithms/Rust/blob/master/src/ciphers/vigenere.rs
//! Vigenère Cipher
//!
//! # Algorithm
//!
//! Rotate each ascii character by the offset of the corresponding key character.
//! When we reach the last key character, we start over from the first one.
//! This implementation does not rotate unicode characters.

/// Vigenère cipher to rotate plain_text text by key and return an owned String.
pub fn vigenere_encode(plain_text: &str, key: &str) -> String {
    // Remove all unicode and non-ascii characters from key
    let key: String = key
        .chars()
        .filter(|&c| c.is_ascii_alphabetic() && !c.is_whitespace())
        .collect();
    let key = key.to_ascii_lowercase();
    println!("Key: {}", key);

    let key_len = key.len();
    if key_len == 0 {
        return String::from(plain_text);
    }

    let mut index = 0;

    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = key.as_bytes()[index % key_len] - b'a';
                index += 1;
                // modulo the distance to keep character range
                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn vigenere_decode(text: &str, key: &str) -> String {
    // Remove all unicode and non-ascii characters from key
    let key: String = key
        .chars()
        .filter(|&c| c.is_ascii_alphabetic() && !c.is_whitespace())
        .collect();
    let key = key.to_ascii_lowercase();
    println!("Key: {}", key);

    let key_len = key.len();
    if key_len == 0 {
        return String::from(text);
    }

    let mut index = 0;

    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_lowercase = c.is_ascii_lowercase();
                let c_lower = c.to_ascii_lowercase();
                // c = "b", first = "a",  shift_by = "b"
                // c = 62, first = 61, shift_by = 62, shift = 1
                let first = b'a';
                let last = b'z';
                let key_byte = key.as_bytes()[index % key_len];
                let shift = key_byte - b'a';
                index += 1;
                // modulo the disactance to keep character range
                // 97 + (101 - 25 - 97) % 26
                if c_lower as u8 - shift < first {
                    let r_shift = (key_byte - c_lower as u8) - 1;
                    return if is_lowercase {
                        (last - r_shift) as char
                    } else {
                        (last - r_shift).to_ascii_uppercase() as char
                    };
                }
                if is_lowercase {
                    (first + (c_lower as u8 - shift - first) % 26) as char
                } else {
                    (first + (c_lower as u8 - shift - first) % 26).to_ascii_uppercase() as char
                }
                // (61 + (62 - 1 - 62) % 26)
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debugging() {
        assert_eq!(vigenere_decode("A", "a"), "A");
        assert_eq!(vigenere_decode("j", "s"), "r");
    }

    #[test]
    fn empty() {
        assert_eq!(vigenere_encode("", "test"), "");
        assert_eq!(vigenere_decode("", "test"), "");
    }

    #[test]
    fn basic() {
        assert_eq!(vigenere_encode("test", "bbbb"), "uftu");
        assert_eq!(vigenere_decode("uftu", "bbbb"), "test");
        assert_eq!(vigenere_decode("uFtu", "bbbb"), "tEst");
    }

    #[test]
    fn vigenere_base() {
        assert_eq!(
            vigenere_encode("LoremIpsumDolorSitAmet", "base"),
            "MojinIhwvmVsmojWjtSqft"
        );
        assert_eq!(
            vigenere_decode("MojinIhwvmVsmojWjtSqft", "base"),
            "LoremIpsumDolorSitAmet"
        );
    }

    #[test]
    fn vigenere_with_spaces() {
        assert_eq!(
            vigenere_encode(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                "spaces"
            ),
            "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx."
        );
        assert_eq!(
            vigenere_decode(
                "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx.",
                "spaces"
            ),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
        );
    }

    #[test]
    fn vigenere_unicode_and_numbers() {
        assert_eq!(
            vigenere_encode("1 Lorem ⏳ ipsum dolor sit amet Ѡ", "unicode"),
            "1 Fbzga ⏳ ltmhu fcosl fqv opin Ѡ"
        );
    }

    #[test]
    fn vigenere_unicode_key() {
        assert_eq!(
            vigenere_encode("Lorem ipsum dolor sit amet", "😉 key!"),
            "Vspoq gzwsw hmvsp cmr kqcd"
        );
    }

    #[test]
    fn vigenere_empty_key() {
        assert_eq!(vigenere_encode("Lorem ipsum", ""), "Lorem ipsum");
    }
}
