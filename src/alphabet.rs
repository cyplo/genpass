bitflags! {
    pub struct Alphabets:u8 {
        const LOWERCASE = 0b0001;
        const UPPERCASE = 0b0010;
        const DIGIT     = 0b0100;
    }
}

impl Default for Alphabets {
    fn default() -> Self {
        Alphabets::all()
    }
}

const LOWERCASE_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE_LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &'static str = "1234567890";

pub fn generate_alphabet(subalphabets: Alphabets) -> Vec<char> {
    let mut resulting_alphabet = Vec::new();
    if subalphabets.contains(Alphabets::UPPERCASE) {
        resulting_alphabet.extend(UPPERCASE_LETTERS.chars());
    }
    if subalphabets.contains(Alphabets::LOWERCASE) {
        resulting_alphabet.extend(LOWERCASE_LETTERS.chars());
    }
    if subalphabets.contains(Alphabets::DIGIT) {
        resulting_alphabet.extend(DIGITS.chars());
    }
    resulting_alphabet
}

#[cfg(test)]
mod must {

    use super::*;
    use std::convert::From;
    use std::ops::Range;

    #[test]
    fn have_all_different_cases_of_letters() {
        assert!(LOWERCASE_LETTERS.eq_ignore_ascii_case(UPPERCASE_LETTERS));
        let alphabet = generate_alphabet(Alphabets::all());
        assert!(
            alphabet
                .iter()
                .any(|character| character.is_ascii_lowercase())
        );
        assert!(
            alphabet
                .iter()
                .any(|character| character.is_ascii_uppercase())
        );
    }

    #[test]
    fn have_all_alphanumeric_characters() {
        let alphabet = generate_alphabet(Alphabets::all());
        let character_code_range: Range<u8> = 0..255;
        let all_alphanumeric_characters: Vec<char> = character_code_range
            .filter(|code| char::from(*code).is_ascii_alphanumeric())
            .map(|code| char::from(code))
            .collect();

        assert!(
            all_alphanumeric_characters
                .iter()
                .all(|character| alphabet.contains(character))
        );
    }

    #[test]
    fn not_be_empty() {
        let alphabet = generate_alphabet(Alphabets::all());
        assert!(!alphabet.is_empty());
    }

    #[test]
    fn not_have_control_codes() {
        let alphabet = generate_alphabet(Alphabets::all());
        assert!(
            alphabet
                .iter()
                .all(|character| !character.is_ascii_control())
        );
    }
}
