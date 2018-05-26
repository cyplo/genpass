use alphabet::Alphabets;
use alphabet::generate_alphabet;
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
            generate_password_from_alphabet(options.length, rng, alphabet.clone());
        let mut password_correct = true;

        password_correct &= any_character_matches(
            alphabets,
            Alphabets::LOWERCASE,
            &password_candidate,
            &|character: char| character.is_ascii_lowercase(),
        );
        password_correct &= any_character_matches(
            alphabets,
            Alphabets::UPPERCASE,
            &password_candidate,
            &|character: char| character.is_ascii_uppercase(),
        );
        password_correct &= any_character_matches(
            alphabets,
            Alphabets::DIGIT,
            &password_candidate,
            &|character: char| character.is_digit(10),
        );
        password_correct &= any_character_matches(
            alphabets,
            Alphabets::SPECIAL,
            &password_candidate,
            &|character: char| !character.is_alphanumeric(),
        );

        if password_correct {
            return password_candidate;
        }
    }
}

fn any_character_matches(
    alphabets: Alphabets,
    alphabet: Alphabets,
    password_candidate: &str,
    character_predicate: &(Fn(char) -> bool),
) -> bool {
    if alphabets.contains(alphabet) {
        password_candidate
            .chars()
            .any(|character| character_predicate(character))
    }
    else {
        true
    }
}

fn generate_password_from_alphabet<Rng: rand::Rng>(
    length: usize,
    rng: &mut Rng,
    alphabet: Vec<char>,
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

    use quickcheck::TestResult;
    use rand::SeedableRng;
    use rand::StdRng;
    use rand::os::OsRng;

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

    quickcheck! {

        fn generate_password_of_given_length(length: usize) -> TestResult {
          if length < MINIMUM_PASSWORD_LENGTH {
            return TestResult::discard();
          }
          let mut rng = OsRng::new().unwrap();
          let password = generate_password(
            GenerationOptions{ length, alphabets: Alphabets::all()},
            &mut rng);
          TestResult::from_bool(password.len() == length)
        }

        fn support_lowercase_letters(length: usize, seed: Vec<usize>) -> TestResult {
          test_generated_password_characters(
            seed,
            GenerationOptions { length, alphabets: Alphabets::all() },
            &|character| character.is_ascii_lowercase() )
        }

        fn support_uppercase_letters(length: usize, seed: Vec<usize>) -> TestResult {
          test_generated_password_characters(
            seed,
            GenerationOptions { length, alphabets: Alphabets::all() },
            &|character| character.is_ascii_uppercase() )
        }

        fn support_digits(length: usize, seed: Vec<usize>) -> TestResult {
          test_generated_password_characters(
            seed,
            GenerationOptions { length, alphabets: Alphabets::all()},
            &|character| character.is_digit(10) )
        }

        fn support_special_chars(length: usize, seed: Vec<usize>) -> TestResult {
            test_generated_password_characters(
                seed,
                GenerationOptions { length, alphabets: Alphabets::all()},
                &|character| character.is_alphanumeric() )
        }


    }

    fn test_generated_password_characters(
        seed: Vec<usize>,
        options: GenerationOptions,
        predicate: &Fn(char) -> bool,
    ) -> TestResult {
        if options.length < MINIMUM_PASSWORD_LENGTH || options.length > MINIMUM_PASSWORD_LENGTH * 2
            || seed.len() == 0
        {
            return TestResult::discard();
        }

        let mut rng: StdRng = SeedableRng::from_seed(seed.as_slice());
        let password = generate_password(options, &mut rng);

        let has_any = password.chars().any(predicate);
        if !has_any {
            eprintln!("Failing password: {}", password);
        }

        TestResult::from_bool(has_any)
    }
}
