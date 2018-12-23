use rand::rngs::OsRng;

mod alphabet;
mod commandline;
mod generator;

use crate::commandline::get_generation_options;
use crate::generator::generate_password;

fn main() {
    let generation_options = get_generation_options();

    let mut rng = OsRng::new().expect("Error opening OS random number generator");
    println!("{}", generate_password(generation_options, &mut rng));
}
