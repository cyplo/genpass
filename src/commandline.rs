use clap::{App, Arg, ArgMatches};
use alphabet::Alphabets;
use generator::GenerationOptions;

const LENGTH_OPTION_NAME: &'static str = "length";
const INCLUDE_LOWERCASE_OPTION_NAME: &'static str = "include-lowercase";
const INCLUDE_UPPERCASE_OPTION_NAME: &'static str = "include-uppercase";
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
                .help("Include at least one lowercase character")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(INCLUDE_UPPERCASE_OPTION_NAME)
                .short("u")
                .help("Include at least one uppercase character")
                .takes_value(false),
        )
        .get_matches();
    options_for_matches(matches)
}

fn options_for_matches(matches: ArgMatches) -> GenerationOptions {
    let length = matches
        .value_of(LENGTH_OPTION_NAME)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let include_lowercase = matches.is_present(INCLUDE_LOWERCASE_OPTION_NAME);
    let include_uppercase = matches.is_present(INCLUDE_UPPERCASE_OPTION_NAME);
    let mut alphabets = Alphabets::NONE;
    if include_lowercase {
        alphabets |= Alphabets::LOWERCASE;
    }
    if include_uppercase {
        alphabets |= Alphabets::UPPERCASE;
    }
    GenerationOptions { length, alphabets }
}

mod must {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn default_to_all_alphabets_when_no_commandline_flags() {
        let matches = ArgMatches {
            args: HashMap::new(),
            subcommand: None,
            usage: None,
        };

        let options = options_for_matches(matches);

        assert!(options.alphabets.is_all());
    }
}
