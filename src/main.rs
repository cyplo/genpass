#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate rand;
use rand::Rng;
use rand::os::OsRng;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let options = App::new("genpass").
        arg(Arg::with_name("length").short("l").index(1).default_value("32")).
        get_matches();
    let length: usize = options.value_of("length").unwrap().parse().unwrap();
    println!("{}", generate_password(length));
}

fn generate_password(length: usize) -> String {
    let mut rng = OsRng::new().expect("Error opening OS random number generator");
    rng.gen_ascii_chars().take(length).collect::<String>()
}

#[cfg(test)]
mod must {
    use super::generate_password;

    quickcheck! {
        fn be_of_given_length(length: usize) -> bool {
          let password = generate_password(length);
          password.len() == length
        }
    }
}
