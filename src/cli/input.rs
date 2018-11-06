use std::io::BufRead;

pub fn get_from_user<R>(mut input: R) -> String where R: BufRead {
    let mut throws = String::new();
    input.read_line(&mut throws)
        .expect("Failed to read line from input");
    throws
}
