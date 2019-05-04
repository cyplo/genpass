use crate::alphabet::generate_alphabet;
use crate::alphabet::Alphabets;
use rand;

const MINIMUM_PASSWORD_LENGTH: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Source {
    Alphabets(Alphabets),
    Words,
}

#[derive(Copy, Clone, Debug)]
pub struct GenerationOptions {
    pub length: usize,
    pub source: Source,
}

pub fn generate_password<Rng: rand::Rng>(options: GenerationOptions, rng: &mut Rng) -> String {
    let length = options.length;
    if length < MINIMUM_PASSWORD_LENGTH {
        panic!(
            "Given the generation options, the length of {} is too short",
            length
        );
    }

    match options.source {
        Source::Alphabets(alphabets) => {
            let alphabet = generate_alphabet(alphabets);

            loop {
                let password_candidate =
                    generate_password_from_alphabet(options.length, rng, alphabet.as_slice());
                if meets_criteria(alphabets, &password_candidate) {
                    return password_candidate;
                }
            }
        }
        Source::Words => "".to_owned(),
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

fn meets_criteria(alphabets: Alphabets, password_candidate: &str) -> bool {
    let mut meets_criteria = true;
    if alphabets.contains(Alphabets::LOWERCASE) {
        meets_criteria &= password_candidate
            .chars()
            .any(|character| character.is_ascii_lowercase());
    }
    if alphabets.contains(Alphabets::UPPERCASE) {
        meets_criteria &= password_candidate
            .chars()
            .any(|character| character.is_ascii_uppercase());
    }
    if alphabets.contains(Alphabets::DIGIT) {
        meets_criteria &= password_candidate
            .chars()
            .any(|character| character.is_digit(10));
    }
    if alphabets.contains(Alphabets::SPECIAL) {
        meets_criteria &= password_candidate
            .chars()
            .any(|character| !character.is_alphanumeric());
    }
    meets_criteria
}

#[cfg(test)]
mod must {
    use super::*;
    use crate::commandline::test::default_commandline_options;
    use proptest::prelude::*;
    use proptest::*;
    use zxcvbn::zxcvbn;

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
                source: Source::Alphabets(Alphabets::all()),
            },
            &mut rng,
        );
    }

    proptest! {
        #[test]
        fn generate_password_of_given_length(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            let password = generate_password_from_all_alphabets(length, seed);

            prop_assert_eq!(password.len(), length)
        }

        #[test]
        #[ignore = "This is a long running test, ignored in dev by default, runs on CI"]
        fn generate_good_passwords_by_default(seed in any::<[u8;32]>()) {
            let length = default_commandline_options().length;
            let password = generate_password_from_all_alphabets(length, seed);

            let estimate = zxcvbn(&password, &[]).unwrap();
            prop_assert_eq!(estimate.score, 4);
        }

        #[test]
        fn support_lowercase_letters(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            test_generated_password_characters(
                length,
                seed,
                &|character| character.is_ascii_lowercase()
            )?
        }

        #[test]
        fn support_uppercase_letters(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            test_generated_password_characters(
                length,
                seed,
                &|character| character.is_ascii_uppercase()
            )?
        }

        #[test]
        fn support_digits(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            test_generated_password_characters(
                length,
                seed,
                &|character| character.is_digit(10)
            )?
        }

        #[test]
        fn support_special_chars(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            test_generated_password_characters(
                length,
                seed,
                &|character| character.is_alphanumeric()
            )?
        }

    }

    fn generate_password_from_all_alphabets(length: usize, seed: [u8; 32]) -> String {
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let options = GenerationOptions {
            length,
            source: Source::Alphabets(Alphabets::all()),
        };
        generate_password(options, &mut rng)
    }

    fn test_generated_password_characters(
        length: usize,
        seed: [u8; 32],
        predicate: &Fn(char) -> bool,
    ) -> Result<(), TestCaseError> {
        let password = generate_password_from_all_alphabets(length, seed);

        prop_assert!(password.chars().any(predicate));
        Ok(())
    }
}
