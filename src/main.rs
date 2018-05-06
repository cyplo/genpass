#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate rand;
use rand::os::OsRng;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let options = App::new("genpass")
        .arg(
            Arg::with_name("length")
                .short("l")
                .index(1)
                .default_value("32"),
        )
        .get_matches();
    let length: usize = options.value_of("length").unwrap().parse().unwrap();

    let mut rng = OsRng::new().expect("Error opening OS random number generator");
    println!(
        "{}",
        generate_password(length, GenerationOptions::Default, &mut rng)
    );
}

fn generate_password<Rng: rand::Rng>(
    length: usize,
    options: GenerationOptions,
    rng: &mut Rng,
) -> String {
    let alphabet = generate_alphabet(options);

    generate_password_from_alphabet(length, rng, alphabet)
}

fn generate_alphabet(options: GenerationOptions) -> Vec<char> {
    let lowercase = "a".chars();
    let uppercase = "A".chars();
    let mut alphabet = Vec::new();
    let options_list = match options {
        GenerationOptions::Default => vec![GenerationOption::Uppercase, GenerationOption::Lowercase],
        GenerationOptions::Custom(list) => list
    };
    for option in options_list {
        let subalphabet = match option {
            GenerationOption::Uppercase => uppercase.clone(),
            GenerationOption::Lowercase => lowercase.clone()
        };
        alphabet.extend(subalphabet);
    }
    alphabet
}

fn generate_password_from_alphabet<Rng: rand::Rng>(
    length: usize,
    rng: &mut Rng,
    mut alphabet: Vec<char>,
) -> String {
    let mut password = String::with_capacity(length);
    while password.len() < length {
        let next_character_index = rng.next_u32() as usize % alphabet.len();
        let next_character = alphabet[next_character_index];
        password.push(next_character);
    }
    password
}

enum GenerationOption {
    Uppercase,
    Lowercase,
}

enum GenerationOptions {
    Default,
    Custom(Vec<GenerationOption>),
}

#[cfg(test)]
mod must {
    use super::*;

    use rand::StdRng;
    use rand::SeedableRng;
    use quickcheck::TestResult;

    quickcheck! {
        fn be_of_given_length(length: usize) -> bool {
          let mut rng = OsRng::new().unwrap();
          let password = generate_password(length, GenerationOptions::Default, &mut rng);
          password.len() == length
        }

        fn support_lowercase_letters(length: usize, seed: Vec<usize>) -> TestResult {
          test_generated_password_characters(
            length,
            seed,
            GenerationOptions::Custom(vec![GenerationOption::Lowercase]),
            &|character| character.is_ascii_lowercase() )
        }

        fn support_uppercase_letters(length: usize, seed: Vec<usize>) -> TestResult {
          test_generated_password_characters(
            length,
            seed,
            GenerationOptions::Custom(vec![GenerationOption::Uppercase]),
            &|character| character.is_ascii_uppercase() )
        }
    }

    fn test_generated_password_characters(
        length: usize,
        seed: Vec<usize>,
        options: GenerationOptions,
        predicate: &Fn(char) -> bool,
    ) -> TestResult {
        if length == 0 || seed.len() == 0 {
            return TestResult::discard();
        }

        let mut rng: StdRng = SeedableRng::from_seed(seed.as_slice());
        let password = generate_password(length, options, &mut rng);

        let result = password.chars().any(predicate);
        TestResult::from_bool(result)
    }
}
