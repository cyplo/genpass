use clap::{App, Arg};
use alphabet::Alphabets;
use generator::GenerationOptions;

pub fn get_generation_options() -> GenerationOptions {
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
    let length = value_t!(options, length_option_name, usize).unwrap_or(32);
    let include_lowercase = options.is_present(include_lowercase_option_name);
    let include_uppercase = options.is_present(include_uppercase_option_name);
    let mut alphabets = Alphabets::NONE;
    if include_lowercase {
        alphabets |= Alphabets::LOWERCASE;
    }
    if include_uppercase {
        alphabets |= Alphabets::UPPERCASE;
    }
    GenerationOptions { length, alphabets }
}

