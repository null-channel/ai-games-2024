// encode a string to morse code
pub fn encode(input: &str) -> String {
    // marek has no idea what he is doing
    let mut output = String::new();

    let trimmed = input.trim();
    // split on word boundaries
    let words: Vec<&str> = trimmed.split(' ').collect();
    println!("{:?}", words);
    for (i, word) in words.iter().enumerate() {
        // split on letter boundaries
        if word.is_empty() {
            output.push_str("0000000");
            continue;
        }
        output.push_str(&encode_word(word));
        if i != words.len() - 1 {
            output.push_str("0000000");
        }
    }
    output
}

pub fn encode_word(input: &str) -> String {
    let mut output = String::new();
    let last_index = input.len() - 1;
    for c in input.char_indices() {
        match c.1.to_ascii_lowercase() {
            'a' => output.push_str("10111"),
            'b' => output.push_str("111010101"),
            'c' => output.push_str("11101011101"),
            'd' => output.push_str("1110101"),
            'e' => output.push_str("1"),
            'f' => output.push_str("101011101"),
            'g' => output.push_str("111011101"),
            'h' => output.push_str("1010101"),
            'i' => output.push_str("101"),
            'j' => output.push_str("1011101110111"),
            'k' => output.push_str("111010111"),
            'l' => output.push_str("101110101"),
            'm' => output.push_str("1110111"),
            'n' => output.push_str("11101"),
            'o' => output.push_str("11101110111"),
            'p' => output.push_str("10111011101"),
            'q' => output.push_str("1110111010111"),
            'r' => output.push_str("1011101"),
            's' => output.push_str("10101"),
            't' => output.push_str("111"),
            'u' => output.push_str("1010111"),
            'v' => output.push_str("101010111"),
            'w' => output.push_str("101110111"),
            'x' => output.push_str("11101010111"),
            'y' => output.push_str("1110101110111"),
            'z' => output.push_str("11101110101"),
            ' ' => output.push_str("0000000"),
            '1' => output.push_str("10111011101110111"),
            '2' => output.push_str("101011101110111"),
            '3' => output.push_str("1010101110111"),
            '4' => output.push_str("10101010111"),
            '5' => output.push_str("101010101"),
            '6' => output.push_str("11101010101"),
            '7' => output.push_str("1110111010101"),
            '8' => output.push_str("111011101110101"),
            '9' => output.push_str("11101110111011101"),
            '0' => output.push_str("1110111011101110111"),
            '.' => output.push_str("1111111"),
            ',' => output.push_str("111111111"),
            _ => output.push_str("I_SHOULD_NOT_BE_HERE"),
        }
        if c.0 != last_index {
            output.push_str("000");
        }
    }
    output
}

pub fn decode_char(input: &str) -> char {
    match input {
        "10111" | "10111" => {
            return 'a';
        }
        "1110101010" | "111010101" => {
            return 'b';
        }
        "111010111010" | "11101011101" => {
            return 'c';
        }
        "11101010" | "1110101" => {
            return 'd';
        }
        "10" | "1" => {
            return 'e';
        }
        "1010111010" | "101011101" => {
            return 'f';
        }
        "1110111010" | "111011101" => {
            return 'g';
        }
        "10101010" | "1010101" => {
            return 'h';
        }
        "1010" | "101" => {
            return 'i';
        }
        "10111011101110" | "1011101110111" => {
            return 'j';
        }
        "1110101110" | "111010111" => {
            return 'k';
        }
        "1011101010" | "101110101" => {
            return 'l';
        }
        "11101110" | "1110111" => {
            return 'm';
        }
        "111010" | "11101" => {
            return 'n';
        }
        "111011101110" | "11101110111" => {
            return 'o';
        }
        "101110111010" | "10111011101" => {
            return 'p';
        }
        "11101110101110" | "1110111010111" => {
            return 'q';
        }
        "10111010" | "1011101" => {
            return 'r';
        }
        "101010" | "10101" => {
            return 's';
        }
        "1110" | "111" => {
            return 't';
        }
        "10101110" | "1010111" => {
            return 'u';
        }
        "1010101110" | "101010111" => {
            return 'v';
        }
        "1011101110" | "101110111" => {
            return 'w';
        }
        "111010101110" | "11101010111" => {
            return 'x';
        }
        "11101011101110" | "1110101110111" => {
            return 'y';
        }
        "111011101010" | "11101110101" => {
            return 'z';
        }
        "0000000" => {
            return ' ';
        }
        "101110111011101110" | "10111011101110111" => {
            return '1';
        }
        "1010111011101110" | "101011101110111" => {
            return '2';
        }
        "10101011101110" | "1010101110111" => {
            return '3';
        }
        "101010101110" | "10101010111" => {
            return '4';
        }
        "1010101010" | "101010101" => {
            return '5';
        }
        "111010101010" | "11101010101" => {
            return '6';
        }
        "11101110101010" | "1110111010101" => {
            return '7';
        }
        "1110111011101010" | "111011101110101" => {
            return '8';
        }
        "111011101110111010" | "11101110111011101" => {
            return '9';
        }
        "11101110111011101110" | "1110111011101110111" => {
            return '0';
        }
        "1111111" => {
            return '.';
        }
        "111111111" => {
            return ',';
        }
        "11111111111" => {
            return '\'';
        }
        _ => {
            return ' ';
        }
    }
}

pub fn decode(input: &str) -> String {
    // spaces are 7 zeros
    // words are separated by 3 zeros
    // letters are separated by 1 zero
    let mut output = String::new();

    // split on word boundaries
    let words: Vec<&str> = input.split("0000000").collect();
    for (i, word) in words.iter().enumerate() {
        if word.is_empty() {
            output.push(' ');
            continue;
        }
        // split on letter boundaries
        let letters: Vec<&str> = word.split("000").collect();
        for letter in letters {
            // split on morse boundaries
            output.push(decode_char(letter));
        }
        if i != words.len() - 1 {
            output.push(' ');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("a"), "10111");
        assert_eq!(encode("aa"), "1011100010111");
        assert_eq!(
            encode("world"),
            "1011101110001110111011100010111010001011101010001110101"
        );
        //TODO: make this test
        //assert_eq!(encode("abcdefghijklmopqrstuvwxyz"), "1011100010111");
    }

    #[test]
    fn test_encode_spaces() {
        assert_eq!(encode(" "), "0000000");
        assert_eq!(encode("a a"), "10111000000010111");
        assert_eq!(encode("a  a"), "101110000000000000010111");
        assert_eq!(
            encode("a world"),
            "1011100000001011101110001110111011100010111010001011101010001110101"
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("10111"), "a");
        assert_eq!(decode("1"), "e");
        assert_eq!(
            decode("1011101110001110111011100010111010001011101010001110101"),
            "world"
        );
    }

    #[test]
    fn test_decode_spaces() {
        assert_eq!(decode("10111000000010111"), "a a");
        assert_eq!(decode("10000000000000010111"), "e  a");
        assert_eq!(
            decode("1011100000001011101110001110111011100010111010001011101010001110101"),
            "a world"
        );
    }
}
