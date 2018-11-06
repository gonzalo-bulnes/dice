use std::io::BufRead;

pub fn get_from_user<R>(mut input: R) -> String where R: BufRead {
    let mut throws = String::new();
    input.read_line(&mut throws)
        .expect("Failed to read line from input");
    throws
}

pub fn parse(throws: &String) -> Vec<&str> {
    let split = throws.split(" ");
    return split.collect();
}
