use crate::alphabet::generate_alphabet;
use crate::alphabet::Alphabets;
use rand;

const MINIMUM_PASSWORD_LENGTH: usize = 4;

#[derive(Copy, Clone)]
pub struct GenerationOptions {
    pub length: usize,
    pub alphabets: Alphabets,
}

pub fn generate_password<Rng: rand::Rng>(options: GenerationOptions, rng: &mut Rng) -> String {
    let alphabets = options.alphabets;
    let length = options.length;
    let alphabet = generate_alphabet(alphabets);

    if length < MINIMUM_PASSWORD_LENGTH {
        panic!(
            "Cannot generate password of length {} given the required criteria",
            length
        );
    }

    loop {
        let password_candidate =
            generate_password_from_alphabet(options.length, rng, alphabet.as_slice());
        let mut password_correct = true;
        if alphabets.contains(Alphabets::LOWERCASE) {
            password_correct &= password_candidate
                .chars()
                .any(|character| character.is_ascii_lowercase());
        }
        if alphabets.contains(Alphabets::UPPERCASE) {
            password_correct &= password_candidate
                .chars()
                .any(|character| character.is_ascii_uppercase());
        }
        if alphabets.contains(Alphabets::DIGIT) {
            password_correct &= password_candidate
                .chars()
                .any(|character| character.is_digit(10));
        }
        if alphabets.contains(Alphabets::SPECIAL) {
            password_correct &= password_candidate
                .chars()
                .any(|character| !character.is_alphanumeric());
        }
        if password_correct {
            return password_candidate;
        }
    }
}

fn generate_password_from_alphabet<Rng: rand::Rng>(
    length: usize,
    rng: &mut Rng,
    alphabet: &[char],
) -> String {
    let mut password = String::with_capacity(length);
    while password.len() < length {
        let next_character_index = rng.next_u32() as usize % alphabet.len();
        let next_character = alphabet[next_character_index];
        password.push(next_character);
    }
    password
}

#[cfg(test)]
mod must {
    use super::*;
    use proptest::prelude::*;
    use proptest::*;

    use rand::rngs::OsRng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use std::ops::Range;

    const MAXIMUM_TESTABLE_PASSWORD_LENGTH: usize = MINIMUM_PASSWORD_LENGTH * 8;
    const TESTABLE_PASSWORD_RANGE: Range<usize> =
        MINIMUM_PASSWORD_LENGTH..MAXIMUM_TESTABLE_PASSWORD_LENGTH;

    #[test]
    #[should_panic]
    fn refuse_to_generate_too_short_of_a_password() {
        let mut rng = OsRng::new().unwrap();
        generate_password(
            GenerationOptions {
                length: MINIMUM_PASSWORD_LENGTH - 1,
                alphabets: Alphabets::all(),
            },
            &mut rng,
        );
    }

    proptest! {
        #[test]
        fn generate_password_of_given_length(length in TESTABLE_PASSWORD_RANGE) {
            let mut rng = OsRng::new().unwrap();
            let password = generate_password(GenerationOptions{ length, alphabets: Alphabets::all()}, &mut rng);
            prop_assert_eq!(password.len(), length)
        }

        #[test]
        fn support_lowercase_letters(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            test_generated_password_characters(
                seed,
                GenerationOptions { length, alphabets: Alphabets::all() },
                &|character| character.is_ascii_lowercase()
            )?
        }

        #[test]
        fn support_uppercase_letters(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            test_generated_password_characters(
                seed,
                GenerationOptions { length, alphabets: Alphabets::all() },
                &|character| character.is_ascii_uppercase()
            )?
        }

        #[test]
        fn support_digits(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            test_generated_password_characters(
                seed,
                GenerationOptions { length, alphabets: Alphabets::all() },
                &|character| character.is_digit(10)
            )?
        }

        #[test]
        fn support_special_chars(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            test_generated_password_characters(
                seed,
                GenerationOptions { length, alphabets: Alphabets::all() },
                &|character| character.is_alphanumeric()
            )?
        }
    }

    fn test_generated_password_characters(
        seed: [u8; 32],
        options: GenerationOptions,
        predicate: &Fn(char) -> bool,
    ) -> Result<(), TestCaseError> {
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let password = generate_password(options, &mut rng);

        prop_assert!(password.chars().any(predicate));
        Ok(())
    }
}
