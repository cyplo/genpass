use crate::alphabet::generate_alphabet;
use crate::alphabet::Alphabets;

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

pub fn generate<Rng: rand::Rng>(options: GenerationOptions, rng: &mut Rng) -> String {
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
                    generate_password_from_alphabet(options.length, rng, &alphabet);
                if meets_criteria(alphabets, &password_candidate) {
                    return password_candidate;
                }
            }
        }
        Source::Words => {
            let mut passphrase = String::with_capacity(length);
            loop {
                passphrase.push_str(eff_wordlist::large::random_word());
                if passphrase.len() >= length {
                    return passphrase;
                }
                passphrase.push(' ');
            }
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
        generate(
            GenerationOptions {
                length: MINIMUM_PASSWORD_LENGTH - 1,
                source: Source::Alphabets(Alphabets::all()),
            },
            &mut OsRng,
        );
    }

    proptest! {
        #[test]
        fn generate_password_of_given_length(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            let password = generate_password_from_all_alphabets(length, seed);

            prop_assert_eq!(password.len(), length)
        }

        #[test]
        fn generate_passphrase_of_given_length_or_longer(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            let passphrase = generate_passphrase(length, seed);

            prop_assert!(passphrase.len() >= length)
        }
    }

    proptest! {
        #[test]
        fn generate_passphrase_with_separator(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            let (_, longest_word) = eff_wordlist::large::LIST.iter().
                max_by(|(_,first_word), (_,second_word)| first_word.len().cmp(&second_word.len())).unwrap();
            prop_assume!(length > longest_word.len() + 1);

            let passphrase = generate_passphrase(length, seed);
            let words : Vec<&str> = passphrase.split(' ').collect();

            prop_assert!(words.len() >1);
        }

        #[test]
        fn generate_passphrase_with_wordlist(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            let passphrase = generate_passphrase(length, seed);
            let words = passphrase.split(' ');

            for word in words {
                prop_assert!(eff_wordlist::large::LIST.iter().any(|(_, rolled_word)| rolled_word.contains(word)), "word {} not in wordlist", word)
            }
        }

        #[test]
        fn generate_passphrase_that_does_not_end_with_separator(length in TESTABLE_PASSWORD_RANGE, seed in any::<[u8;32]>()) {
            let passphrase = generate_passphrase(length, seed);

            prop_assert!(!passphrase.ends_with(' '));
        }
    }

    proptest! {
        #[test]
        #[ignore = "This is a long running test, ignored in dev by default, runs on CI"]
        fn generate_good_passwords_by_default(seed in any::<[u8;32]>()) {
            let length = default_commandline_options().length;
            let password = generate_password_from_all_alphabets(length, seed);

            let estimate = zxcvbn(&password, &[]).unwrap();
            prop_assert_eq!(estimate.score(), 4);
        }

        #[test]
        #[ignore = "This is a long running test, ignored in dev by default, runs on CI"]
        fn generate_good_passphrases_by_default(seed in any::<[u8;32]>()) {
            let length = default_commandline_options().length;
            let passphrase = generate_passphrase(length, seed);

            let estimate = zxcvbn(&passphrase, &[]).unwrap();
            prop_assert_eq!(estimate.score(), 4);
        }
    }

    proptest! {
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

    fn generate_passphrase(length: usize, seed: [u8; 32]) -> String {
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let options = GenerationOptions {
            length,
            source: Source::Words,
        };
        generate(options, &mut rng)
    }

    fn generate_password_from_all_alphabets(length: usize, seed: [u8; 32]) -> String {
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let options = GenerationOptions {
            length,
            source: Source::Alphabets(Alphabets::all()),
        };
        generate(options, &mut rng)
    }

    fn test_generated_password_characters(
        length: usize,
        seed: [u8; 32],
        predicate: &dyn Fn(char) -> bool,
    ) -> Result<(), TestCaseError> {
        let password = generate_password_from_all_alphabets(length, seed);

        prop_assert!(password.chars().any(predicate));
        Ok(())
    }
}
