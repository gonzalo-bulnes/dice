extern crate dice;

fn run_app() -> Result<(), String> {
    use std::io;
    use std::io::Write;
    use dice::wordlist;

    println!("Welcome to 🎲 Dice!");
    println!("v0.1.0\n");

    println!("--------------------------------------------------------------------\n");
    println!("This little program will give you the passphrase that corresponds to");
    println!("the result of your dice throws, using EFF's long wordlist.\n");
    println!("Haven't thrown your dice yet?\nPlease visit https://www.eff.org/dice for guidance!\n");
    println!("You can lookup at once as many dice throws as you want,\njust separe them with a space.\n");
    println!("Example:\n\n   Dice throws: 11111 22222 33333 44444 55555 66666\n");
    println!("--------------------------------------------------------------------\n");


    // get user input
    let mut throws = String::new();
    print!("Dice throws: ");
    io::stdout().flush().expect("Failed to write line");
    io::stdin().read_line(&mut throws)
        .expect("Failed to read line");

    // convert string to valid list of numbers
    let split = throws.split(" ");
    let throws: Vec<&str> = split.collect();

    // collect errors for friendly output
    let mut messages: Vec<String> = Vec::with_capacity(throws.len());

    let throws: Vec<Option<u32>> = throws.iter().map(|&t|
        match wordlist::parse_throw(t) {
            Ok(throw) => Some(throw),
            Err(error) => match error {
               wordlist::Error::IsNotNumber(input) => {
                   messages.push(format!("Dice throws must be composed of numbers, '{}' looks like a typo.", input.trim()));
                   None
               },
               wordlist::Error::IsNotDieValue(input) => {
                   messages.push(format!("Only dice values between 1 and 6 are valid, value '{}' in '{}' looks like a typo.", input, t.trim()));
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
    println!("Passphrase:  {}", dice::passphrase(&words));

    // display error messages if any
    if messages.len() > 0 {
        let prefixed_messages: Vec<String> = messages.iter().map(|m| format!("    - {}", m)).collect();
        return Err(prefixed_messages.join("\n"))
    }
    Ok(())
}

fn main() {
    ::std::process::exit(match run_app() {
       Ok(_) => 0,
       Err(err) => {
           eprintln!("\nPlease note:\n\n{}", err);
           1
       }
    });
}
