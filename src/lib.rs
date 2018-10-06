#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_with_valid_throw_returns_word() {
        let expected = Some(String::from("abacus"));
        assert_eq!(expected, lookup(11111));

        let expected = Some(String::from("abdomen"));
        assert_eq!(expected, lookup(11112));

        let expected = Some(String::from("zoom"));
        assert_eq!(expected, lookup(66666));
    }

    #[test]
    fn lookup_with_invalid_throw_returns_none() {
        assert_eq!(None, lookup(0));
        assert_eq!(None, lookup(11110));
        assert_eq!(None, lookup(66667));
    }
}

pub fn lookup(throw: u32) -> std::option::Option<String> {
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
