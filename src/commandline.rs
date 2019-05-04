use crate::alphabet::Alphabets;
use crate::generator::{GenerationOptions, Source};
use structopt::StructOpt;

#[derive(Copy, Clone, StructOpt, Debug)]
#[structopt(name = "genpass")]
pub struct CommandlineOptions {
    #[structopt(long = "version")]
    pub print_version: bool,

    #[structopt(
        long = "length",
        help = "The length of the password to generate",
        default_value = "32"
    )]
    pub length: usize,

    #[structopt(
        short = "l",
        long = "include-lowercase",
        help = "Include at least one lowercase letter"
    )]
    include_lowercase: bool,
    #[structopt(
        short = "u",
        long = "include-uppercase",
        help = "Include at least one uppercase letter"
    )]
    include_uppercase: bool,
    #[structopt(
        short = "d",
        long = "include-digit",
        help = "Include at least one digit"
    )]
    include_digit: bool,
    #[structopt(
        short = "s",
        long = "include-special",
        help = "Include at least one special (non-alphanumericc) character"
    )]
    include_special: bool,
}

pub fn generation_options_for_commandline_options(
    options: CommandlineOptions,
) -> GenerationOptions {
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
    if options.include_special {
        alphabets |= Alphabets::SPECIAL;
    }
    if alphabets.is_empty() {
        alphabets = Alphabets::all();
    }
    GenerationOptions {
        length: options.length,
        source: Source::Alphabets(alphabets),
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn default_commandline_options() -> CommandlineOptions {
        let empty_args: [String; 0] = [];
        CommandlineOptions::from_iter(&empty_args)
    }
}

#[cfg(test)]
mod must {

    use super::*;

    #[test]
    fn support_lowercase_letters() {
        let commandline_options = CommandlineOptions {
            print_version: false,
            length: 0,
            include_lowercase: true,
            include_uppercase: false,
            include_digit: false,
            include_special: false,
        };

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::LOWERCASE);
    }

    #[test]
    fn support_uppercase_letters() {
        let commandline_options = CommandlineOptions {
            print_version: false,
            length: 0,
            include_lowercase: false,
            include_uppercase: true,
            include_digit: false,
            include_special: false,
        };

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::UPPERCASE);
    }

    #[test]
    fn support_digits() {
        let commandline_options = CommandlineOptions {
            print_version: false,
            length: 0,
            include_lowercase: false,
            include_uppercase: false,
            include_digit: true,
            include_special: false,
        };

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::DIGIT);
    }

    #[test]
    fn support_special_chars() {
        let commandline_options = CommandlineOptions {
            print_version: false,
            length: 0,
            include_lowercase: false,
            include_uppercase: false,
            include_digit: false,
            include_special: true,
        };

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::SPECIAL);
    }

    #[test]
    fn default_to_all_alphabets_when_no_commandline_flags() {
        let commandline_options = CommandlineOptions {
            print_version: false,
            length: 0,
            include_lowercase: false,
            include_uppercase: false,
            include_special: false,
            include_digit: false,
        };

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::all());
    }

    fn assert_is_subalphabet(actual: Source, expected: Alphabets) {
        match actual {
            Source::Alphabets(alphabets) => {
                assert_eq!(alphabets, expected);
            }
            Source::Words(_) => assert!(false),
        }
    }
}
