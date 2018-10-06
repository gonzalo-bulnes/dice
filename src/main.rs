fn main() {
    use std::io;

    println!("Welcome to Dice!");

    // get user input
    let mut throws = String::new();
    io::stdin().read_line(&mut throws)
        .expect("Failed to read line");

    // convert string to list of numbers
    let split = throws.split(" ");
    let throws: Vec<&str> = split.collect();
    let throws: Vec<u32> = throws.iter().map(|&t|
        parse_throw(t)
    ).collect();

    for s in &throws {
        println!("{}", s)
    }
}

fn parse_throw(throw: &str) -> u32 {
    throw.trim().parse().expect("Please type a number!")
}
