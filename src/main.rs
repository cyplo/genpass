use rand::rngs::OsRng;

mod alphabet;
mod built_info;
mod commandline;
mod generator;

use crate::generator::generate_password;
use structopt::StructOpt;

fn main() {
    let options = commandline::CommandlineOptions::from_args();

    if options.print_version {
        println!(
            "This is genpass {}{}, built on {} by {}.",
            built_info::PKG_VERSION,
            built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" ({})", v)),
            built_info::BUILT_TIME_UTC,
            built_info::RUSTC_VERSION
        );
        return;
    }

    let mut rng = OsRng::new().expect("Error opening OS random number generator");
    let generation_options = commandline::generation_options_for_commandline_options(options);
    println!("{}", generate_password(generation_options, &mut rng));
}
