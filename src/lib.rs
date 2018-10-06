#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_with_valid_throw_returns_word() {
        let expected = Some(std::string::String::from("abacus"));
        assert_eq!(expected, lookup(11111));

        let expected = Some(std::string::String::from("abdomen"));
        assert_eq!(expected, lookup(11112));

        let expected = Some(std::string::String::from("zoom"));
        assert_eq!(expected, lookup(66666));
    }

    #[test]
    fn lookup_with_invalid_throw_returns_none() {
        assert_eq!(None, lookup(0));
        assert_eq!(None, lookup(11110));
        assert_eq!(None, lookup(66667));
    }
}

fn lookup(throw: u32) -> std::option::Option<std::string::String> {
    use std::collections::HashMap;

    let throws = vec![11111, 11112, 66666];
    let words = vec![String::from("abacus"), String::from("abdomen"), String::from("zoom")];

    let eff_long_wordlist: HashMap<_, _> = throws.iter().zip(words.iter()).collect();

    let word = eff_long_wordlist.get(&throw);
    match word {
        None => None,
        Some(s) => Some((**s).clone()),
    }
}
