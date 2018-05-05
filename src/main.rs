#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate rand;
use rand::Rng;
use rand::os::OsRng;

fn main() {
    println!("{}", generate_password(32));
}

fn generate_password(length: usize) -> String {
    let mut rng = OsRng::new().expect("Error opening OS random number generator");
    rng.gen_ascii_chars().take(length).collect::<String>()
}

#[cfg(test)]
mod must_be {
    use super::generate_password;

    quickcheck! {
        fn of_given_length(length: usize) -> bool {
          let password = generate_password(length);
          password.len() == length
        }
    }
}
