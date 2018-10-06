extern crate dice;

fn run_app() -> Result<(), String> {
    use std::io;
    use std::io::Write;
    use dice::wordlist;

    println!("Welcome to Dice!");

    // get user input
    let mut throws = String::new();
    print!("Dice throws (as many as you want): ");
    io::stdout().flush().expect("Failed to write line");
    io::stdin().read_line(&mut throws)
        .expect("Failed to read line");

    // convert string to valid list of numbers
    let split = throws.split(" ");
    let throws: Vec<&str> = split.collect();
    let throws: Vec<Option<u32>> = throws.iter().map(|&t|
        match wordlist::parse_throw(t) {
            Ok(throw) => Some(throw),
            Err(error) => match error {
               wordlist::Error::IsNotNumber(input) => {
                   eprintln!("Dice throws must be composed of numbers, '{}' looks like a typo.", input.trim());
                   None
               },
               wordlist::Error::IsNotDieValue(input) => {
                   eprintln!("Only dice values between 1 and 6 are valid, value '{}' in '{}' looks like a typo.", input, t.trim());
                   None
               }
            },
       }
    ).collect();

    // lookup words based on the dice throws
    let words = throws.iter().map(|&throw|
        wordlist::lookup(throw)
    ).collect();

    // display passphrase
    println!("Passphrase: {}", dice::passphrase(&words));
    Ok(())
}

fn main() {
    ::std::process::exit(match run_app() {
       Ok(_) => 0,
       Err(err) => {
           eprintln!("error: {:?}", err);
           1
       }
    });
}
