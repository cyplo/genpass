use rand;
use alphabet::Alphabets;
use alphabet::generate_alphabet;

#[derive(Copy, Clone)]
pub struct GenerationOptions {
    pub length: usize,
    pub alphabets: Alphabets,
}

pub fn generate_password<Rng: rand::Rng>(options: GenerationOptions, rng: &mut Rng) -> String {
    let alphabet = generate_alphabet(options.alphabets);

    generate_password_from_alphabet(options.length, rng, alphabet)
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

    use rand::StdRng;
    use rand::SeedableRng;
    use rand::os::OsRng;
    use quickcheck::TestResult;

    quickcheck! {
        fn generate_password_of_given_length(length: usize) -> bool {
          let mut rng = OsRng::new().unwrap();
          let password = generate_password(
            GenerationOptions{ length, alphabets: Alphabets::ALL },
            &mut rng);
          password.len() == length
        }

        fn support_lowercase_letters(length: usize, seed: Vec<usize>) -> TestResult {
          test_generated_password_characters(
            seed,
            GenerationOptions { length, alphabets: Alphabets::LOWERCASE },
            &|character| character.is_ascii_lowercase() )
        }

        fn support_uppercase_letters(length: usize, seed: Vec<usize>) -> TestResult {
          test_generated_password_characters(
            seed,
            GenerationOptions { length, alphabets: Alphabets::UPPERCASE },
            &|character| character.is_ascii_uppercase() )
        }
    }

    fn test_generated_password_characters(
        seed: Vec<usize>,
        options: GenerationOptions,
        predicate: &Fn(char) -> bool,
    ) -> TestResult {
        if options.length == 0 || seed.len() == 0 {
            return TestResult::discard();
        }

        let mut rng: StdRng = SeedableRng::from_seed(seed.as_slice());
        let password = generate_password(options, &mut rng);

        let result = password.chars().any(predicate);
        TestResult::from_bool(result)
    }
}
