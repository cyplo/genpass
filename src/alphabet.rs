use bitflags::bitflags;

bitflags! {
    pub struct Alphabets:u8 {
        const LOWERCASE = 0b0001;
        const UPPERCASE = 0b0010;
        const DIGIT     = 0b0100;
        const SPECIAL   = 0b1000;
    }
}

impl Default for Alphabets {
    fn default() -> Self {
        Alphabets::all()
    }
}

const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS_CHARS: &str = "1234567890";
const SPECIAL_CHARS: &str = "~!@#$%^&*()-=_+{}[]|\';/.,?><";

pub fn generate_alphabet(subalphabets: Alphabets) -> Vec<char> {
    let mut resulting_alphabet = Vec::new();
    if subalphabets.contains(Alphabets::UPPERCASE) {
        resulting_alphabet.extend(UPPERCASE_CHARS.chars());
    }
    if subalphabets.contains(Alphabets::LOWERCASE) {
        resulting_alphabet.extend(LOWERCASE_CHARS.chars());
    }
    if subalphabets.contains(Alphabets::DIGIT) {
        resulting_alphabet.extend(DIGITS_CHARS.chars());
    }
    if subalphabets.contains(Alphabets::SPECIAL) {
        resulting_alphabet.extend(SPECIAL_CHARS.chars());
    }
    resulting_alphabet
}

#[cfg(test)]
mod must {

    use super::*;
    use std::convert::From;

    #[test]
    fn have_all_different_cases_of_letters() {
        assert!(LOWERCASE_CHARS.eq_ignore_ascii_case(UPPERCASE_CHARS));
        let alphabet = generate_alphabet(Alphabets::all());
        assert!(alphabet
            .iter()
            .any(|character| character.is_ascii_lowercase()));
        assert!(alphabet
            .iter()
            .any(|character| character.is_ascii_uppercase()));
    }

    #[test]
    fn have_all_alphanumeric_characters() {
        let alphabet = generate_alphabet(Alphabets::all());
        let character_code_range = 0..=255;
        let all_alphanumeric_characters: Vec<char> = character_code_range
            .filter(|code| char::from(*code).is_ascii_alphanumeric())
            .map(|code| char::from(code))
            .collect();
        assert!(all_alphanumeric_characters
            .iter()
            .all(|character| alphabet.contains(character)));
    }

    #[test]
    fn not_be_empty() {
        let alphabet = generate_alphabet(Alphabets::all());
        assert!(!alphabet.is_empty());
    }

    #[test]
    fn not_have_control_codes() {
        let alphabet = generate_alphabet(Alphabets::all());
        assert!(alphabet
            .iter()
            .all(|character| !character.is_ascii_control()));
    }

    #[test]
    fn have_special_chars() {
        let alphabet = generate_alphabet(Alphabets::SPECIAL);
        assert!(alphabet
            .iter()
            .all(|character| !character.is_alphanumeric()));
    }

}
