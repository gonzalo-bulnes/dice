mod messages;

use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

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

pub fn welcome<W>(mut output: W) where W: Write {
    write!(&mut output, "{}", messages::WELCOME).expect("Unable to write to output");
}
