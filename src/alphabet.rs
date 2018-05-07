bitflags! {
    pub struct Alphabets:u8 {
        const NONE      = 0;
        const LOWERCASE = 2^0;
        const UPPERCASE = 2^1;
        const ALL = u8::max_value();
    }
}

pub fn generate_alphabet(subalphabets: Alphabets) -> Vec<char> {
    let lowercase = "a".chars();
    let uppercase = "A".chars();
    let mut resulting_alphabet = Vec::new();
    if subalphabets.contains(Alphabets::UPPERCASE) {
        resulting_alphabet.extend(uppercase);
    }
    if subalphabets.contains(Alphabets::LOWERCASE) {
        resulting_alphabet.extend(lowercase);
    }
    resulting_alphabet
}

#[cfg(test)]
mod must {
    use super::*;

    use rand::StdRng;
    use rand::SeedableRng;
    use quickcheck::TestResult;

    quickcheck! {
        fn be_all_when_none_specified() -> bool {
            false
        }
    }

}
