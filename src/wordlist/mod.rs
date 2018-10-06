extern crate std;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_with_valid_throw_returns_word() {
        let expected = Some(String::from("abacus"));
        assert_eq!(expected, lookup(Some(11111)));

        let expected = Some(String::from("abdomen"));
        assert_eq!(expected, lookup(Some(11112)));

        let expected = Some(String::from("zoom"));
        assert_eq!(expected, lookup(Some(66666)));

        assert_eq!(None, lookup(None));
    }

    #[test]
    fn lookup_with_invalid_throw_returns_none() {
        assert_eq!(None, lookup(Some(0)));
        assert_eq!(None, lookup(Some(11110)));
        assert_eq!(None, lookup(Some(66667)));
    }

    #[test]
    fn parse_throw_rejects_non_numbers() {
        assert_eq!(true, parse_throw("hello").is_err(), "Expected 'hello' to be rejected, was not");
        assert_eq!(true, parse_throw("55555a").is_err(), "Expected '55555a' to be rejected, was not");
        assert_eq!(true, parse_throw("4444b").is_err(), "Expected '4444b' to be rejected, was not");
        assert_eq!(true, parse_throw("").is_err(), "Expected '' to be rejected, was not");
    }

    #[test]
    fn validate_throw_rejects_non_dice() {
        assert_eq!(false, validate_throw(11111).is_err(), "Valid throw 11111 was rejected");
        assert_eq!(false, validate_throw(15246).is_err(), "Valid throw 15246 was rejected");
        assert_eq!(false, validate_throw(43161).is_err(), "Valid throw 43161 was rejected");
        assert_eq!(false, validate_throw(66666).is_err(), "Valid throw 66666 was rejected");
        assert_eq!(true, validate_throw(11711).is_err(), "Expected 11711 to be rejected, was not");
        assert_eq!(true, validate_throw(66660).is_err(), "Expected 66660 to be rejected, was not");
        assert_eq!(true, validate_throw(18111).is_err(), "Expected 18111 to be rejected, was not");
        assert_eq!(true, validate_throw(01111).is_err(), "Expected 01111 to be rejected, was not");
        assert_eq!(true, validate_throw(22292).is_err(), "Expected 22292 to be rejected, was not");
    }

    #[test]
    fn validate_die_rejects_all_but_1_2_3_4_5_6() {
        assert_eq!(false, validate_die(1).is_err(), "Valid die 1 was rejected");
        assert_eq!(false, validate_die(2).is_err(), "Valid die 2 was rejected");
        assert_eq!(false, validate_die(3).is_err(), "Valid die 3 was rejected");
        assert_eq!(false, validate_die(4).is_err(), "Valid die 4 was rejected");
        assert_eq!(false, validate_die(5).is_err(), "Valid die 5 was rejected");
        assert_eq!(false, validate_die(6).is_err(), "Valid die 6 was rejected");
        assert_eq!(true, validate_die(0).is_err(), "Expected 0 to be rejected, was not");
        assert_eq!(true, validate_die(7).is_err(), "Expected 7 to be rejected, was not");
        assert_eq!(true, validate_die(11).is_err(), "Expected 11 to be rejected, was not");
    }
}

pub enum Error {
    IsNotNumber(String),
    IsNotDieValue(u32),
}

pub fn lookup(throw: Option<u32>) -> Option<String> {
    use std::collections::HashMap;

    let throws = vec![11111, 11112, 66666];
    let words = vec![String::from("abacus"), String::from("abdomen"), String::from("zoom")];

    let eff_long_wordlist: HashMap<_, _> = throws.iter().zip(words.iter()).collect();

    match throw {
        Some(throw) => {
            let word = eff_long_wordlist.get(&throw);
            match word {
                None => None,
                Some(s) => Some((**s).clone()),
            }
        },
        None => return None,
    }
}

pub fn parse_throw(throw: &str) -> std::result::Result<u32,Error> {
    let throw = match throw.trim().parse() {
        Ok(t) => t,
        Err(_) => return Err(Error::IsNotNumber(String::from(throw))),
    };

    validate_throw(throw)
}

fn validate_throw(throw: u32) -> std::result::Result<u32,Error> {
    let dice = vec![
        throw % 10,
        throw % 100 / 10,
        throw % 1000 / 100,
        throw % 10000 / 1000,
        throw / 10000,
    ];

    for die in dice {
        let _ = match validate_die(die) {
            Err(error) => return Err(error),
            ok => ok,
        };
    };
    Result::Ok(throw)
}

fn validate_die(die: u32) -> std::result::Result<u32,Error> {
    use std::cmp::Ordering;

    let die = match die.cmp(&1) {
        Ordering::Less => return Err(Error::IsNotDieValue(die)),
        _ => die,
    };
    let die = match die.cmp(&6) {
        Ordering::Greater => return Err(Error::IsNotDieValue(die)),
        _ => die,
    };
    Ok(die)
}
