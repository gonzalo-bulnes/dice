pub mod cli;
pub mod wordlist;

const PLACEHOLDER: &str = "-----";
const SEPARATOR: &str = " ";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passphrase_joins_words_with_single_space() {
        assert_eq!("", passphrase(&vec![]), "Expected '{}', got '{}'", "", passphrase(&vec![]));

        let words = vec![
            None,
        ];
        assert_eq!("-----", passphrase(&words), "Expected '{}', got '{}'", "", passphrase(&words));

        let words = vec![
            Some(String::from("abacus")),
        ];
        assert_eq!("abacus", passphrase(&words), "Expected '{}', got '{}'", "abacus", passphrase(&words));

        let words = vec![
            Some(String::from("abacus")),
            Some(String::from("zoom")),
        ];
        assert_eq!("abacus zoom", passphrase(&words), "Expected '{}', got '{}'", "abacus zoom", passphrase(&words));

        let words = vec![
            Some(String::from("abacus")),
            Some(String::from("zoom")),
            Some(String::from("banana")),
        ];
        assert_eq!("abacus zoom banana", passphrase(&words), "Expected '{}', got '{}'", "abacus zoom banana", passphrase(&words));

        let words = vec![
            Some(String::from("abacus")),
            None,
            Some(String::from("zoom")),
            Some(String::from("banana")),
            None,
        ];
        assert_eq!("abacus ----- zoom banana -----", passphrase(&words), "Expected '{}', got '{}'", "abacus zoom banana", passphrase(&words));
    }
}

pub fn passphrase(words: &Vec<Option<String>>) -> String {
    let words: Vec<String> = words.iter().map(|word|
        match word {
            Some(word) => word.clone(),
            None => String::from(PLACEHOLDER),
        }
    ).collect();

    let mut words = words.clone();
    words.retain(|word| !word.is_empty());
    words.join(SEPARATOR)
}
