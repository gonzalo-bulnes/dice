use std::io::Write;
use cli::messages;

pub fn passphrase<W>(mut output: W, passphrase: &str) where W: Write {
    write!(&mut output, "{}{}", messages::PASSWORD_PREFIX, passphrase).expect("Unable to write to output");
    return
}

pub fn prompt_for_user_input<W>(mut output: W,) where W: Write {
    write!(&mut output, "{}", messages::PROMPT).expect("Unable to write to output");
    output.flush().expect("Unable to write to output");
}

pub fn welcome<W>(mut output: W) where W: Write {
    write!(&mut output, "{}", messages::WELCOME).expect("Unable to write to output");
}
