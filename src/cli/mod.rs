mod input;
mod messages;
mod output;

use std::io::BufRead;
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_passphrase_prints_it_to_output() {
        let mut output = Vec::new();

        display_passphrase(&mut output, "correct horse battery staple");

        let output = String::from_utf8(output).expect("Expected UTF-8 output, was not");

        assert!(output.contains("correct horse battery staple"),
            "Expected output to contain passphrase, did not");
    }

    #[test]
    fn get_user_input_works() {
        let input = b"12345 65432";

        let captured = get_user_input(&input[..]);
        assert_eq!(String::from("12345 65432"), captured, "Expected input to be captured, was not");
    }

    #[test]
    fn prompt_prints_prompt_to_output() {
        let mut output = Vec::new();

        prompt_for_user_input(&mut output);

        let output = String::from_utf8(output).expect("Expected UTF-8 output, was not");

        assert!(output.contains("Dice"),
            "Expected output to contain prompt for dice throws, did not");
    }

    #[test]
    fn welcome_prints_basic_guidance_to_output() {
        let mut output = Vec::new();

        welcome(&mut output);

        let output = String::from_utf8(output).expect("Expected UTF-8 output, was not");

        assert!(output.contains("https://www.eff.org/dice"),
            "Expected output to contain guidance on generating passphrases using dice, did not");
    }

    #[test]
    fn welcome_prints_version_number_to_output() {
        let mut output = Vec::new();

        welcome(&mut output);

        let output = String::from_utf8(output).expect("Expected UTF-8 output, was not");

        assert!(output.contains("v1.0.0-alpha"),
            "Expected output to contain Dice version number, did not");
    }
}

pub fn display_passphrase<W>(output: W, passphrase: &str) where W: Write {
    output::passphrase(output, passphrase);
}

pub fn get_user_input<R>(input: R) -> String where R: BufRead {
    return input::get_from_user(input);
}

pub fn prompt_for_user_input<W>(output: W,) where W: Write {
    output::prompt_for_user_input(output);
}

pub fn welcome<W>(output: W) where W: Write {
    output::welcome(output);
}
