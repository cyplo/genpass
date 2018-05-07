#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate rand;
use rand::os::OsRng;

#[macro_use]
extern crate bitflags;

extern crate clap;

mod commandline;
use commandline::get_generation_options;

mod alphabet;

mod generator;
use generator::generate_password;

fn main() {
    let generation_options = get_generation_options();

    let mut rng = OsRng::new().expect("Error opening OS random number generator");
    println!("{}", generate_password(generation_options, &mut rng));
}
