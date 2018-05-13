use alphabet::Alphabets;
use clap::{App, Arg, ArgMatches};
use generator::GenerationOptions;

const LENGTH_OPTION_NAME: &'static str = "length";
const INCLUDE_LOWERCASE_OPTION_NAME: &'static str = "include-lowercase";
const INCLUDE_UPPERCASE_OPTION_NAME: &'static str = "include-uppercase";
const INCLUDE_DIGIT_OPTION_NAME: &'static str = "include-digit";
const DEFAULT_LENGTH: &'static str = "32";

pub fn get_generation_options() -> GenerationOptions {
    let matches = App::new("genpass")
        .arg(
            Arg::with_name(LENGTH_OPTION_NAME)
                .short("l")
                .index(1)
                .default_value(DEFAULT_LENGTH),
        )
        .arg(
            Arg::with_name(INCLUDE_LOWERCASE_OPTION_NAME)
                .short("l")
                .help("Include at least one lowercase letter")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(INCLUDE_UPPERCASE_OPTION_NAME)
                .short("u")
                .help("Include at least one uppercase letter")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(INCLUDE_DIGIT_OPTION_NAME)
                .short("d")
                .help("Include at least one digit")
                .takes_value(false),
        )
        .get_matches();
    let commandline_options = commandline_options_for_matches(matches);
    generation_options_for_commandline_options(commandline_options)
}

#[derive(Copy, Clone)]
struct CommandlineOptions {
    length: usize,
    include_lowercase: bool,
    include_uppercase: bool,
    include_digit: bool,
}

fn commandline_options_for_matches(matches: ArgMatches) -> CommandlineOptions {
    let length = matches
        .value_of(LENGTH_OPTION_NAME)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let include_lowercase = matches.is_present(INCLUDE_LOWERCASE_OPTION_NAME);
    let include_uppercase = matches.is_present(INCLUDE_UPPERCASE_OPTION_NAME);
    let include_digit = matches.is_present(INCLUDE_DIGIT_OPTION_NAME);
    CommandlineOptions {
        length,
        include_lowercase,
        include_uppercase,
        include_digit,
    }
}

fn generation_options_for_commandline_options(options: CommandlineOptions) -> GenerationOptions {
    let mut alphabets = Alphabets::empty();
    if options.include_lowercase {
        alphabets |= Alphabets::LOWERCASE;
    }
    if options.include_uppercase {
        alphabets |= Alphabets::UPPERCASE;
    }
    if options.include_digit {
        alphabets |= Alphabets::DIGIT;
    }
    if alphabets.is_empty() {
        alphabets = Alphabets::all();
    }
    GenerationOptions {
        length: options.length,
        alphabets,
    }
}

#[cfg(test)]
mod must {

    use super::*;

    #[test]
    fn support_lowercase_letters() {
        let commandline_options = CommandlineOptions {
            length: 0,
            include_lowercase: true,
            include_uppercase: false,
            include_digit: false,
        };

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_eq!(generation_options.alphabets, Alphabets::LOWERCASE);
    }

    #[test]
    fn support_uppercase_letters() {
        let commandline_options = CommandlineOptions {
            length: 0,
            include_lowercase: false,
            include_uppercase: true,
            include_digit: false,
        };

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_eq!(generation_options.alphabets, Alphabets::UPPERCASE);
    }

    #[test]
    fn support_digits() {
        let commandline_options = CommandlineOptions {
            length: 0,
            include_lowercase: false,
            include_uppercase: false,
            include_digit: true,
        };

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_eq!(generation_options.alphabets, Alphabets::DIGIT);
    }

    #[test]
    fn default_to_all_alphabets_when_no_commandline_flags() {
        let commandline_options = CommandlineOptions {
            length: 0,
            include_lowercase: false,
            include_uppercase: false,
            include_digit: false,
        };

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert!(generation_options.alphabets.is_all());
    }
}
