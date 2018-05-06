#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate rand;
use rand::os::OsRng;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let commandline_options = parse_commandline_options();
    let generation_options = get_generation_options(commandline_options);

    let mut rng = OsRng::new().expect("Error opening OS random number generator");
    println!(
        "{}",
        generate_password(commandline_options.length, generation_options, &mut rng)
    );
}

fn get_generation_options(commandline_options: CommandlineOptions) -> GenerationOptions {
    if !commandline_options.include_lowercase
        && !commandline_options.include_uppercase {
        return GenerationOptions::Default
    }
    let mut options = Vec::new();
    if commandline_options.include_lowercase { options.push(GenerationOption::Lowercase);}
    if commandline_options.include_uppercase { options.push(GenerationOption::Uppercase);}
    GenerationOptions::Custom(options)
}

#[derive(Copy, Clone)]
struct CommandlineOptions {
    length: usize,
    include_lowercase: bool,
    include_uppercase: bool
}

fn parse_commandline_options() -> CommandlineOptions {
    let length_option_name = "length";
    let include_lowercase_option_name = "include-lowercase";
    let include_uppercase_option_name = "include-uppercase";
    let options = App::new("genpass")
        .arg(
            Arg::with_name(length_option_name)
                .short("l")
                .index(1)
                .default_value("32"),
        )
        .arg(
            Arg::with_name(include_lowercase_option_name)
                .short("l")
                .help("Include at least one lowercase character")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(include_uppercase_option_name)
                .short("u")
                .help("Include at least one uppercase character")
                .takes_value(false),
        )
        .get_matches();
    let length: usize = options.value_of(length_option_name).unwrap().parse().unwrap();
    let include_lowercase = options.is_present(include_lowercase_option_name);
    let include_uppercase = options.is_present(include_uppercase_option_name);
    CommandlineOptions { length, include_lowercase, include_uppercase }
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
        GenerationOptions::Default => {
            vec![GenerationOption::Uppercase, GenerationOption::Lowercase]
        }
        GenerationOptions::Custom(list) => list,
    };
    for option in options_list {
        let subalphabet = match option {
            GenerationOption::Uppercase => uppercase.clone(),
            GenerationOption::Lowercase => lowercase.clone(),
        };
        alphabet.extend(subalphabet);
    }
    alphabet
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
