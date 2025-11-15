
mod greeter;
mod voter;

use std::io;
use greeter::greet;
use voter::log_voter;

fn main() {
    println!("Naija Voter Checker (Type 'quit' to exit)\n");

    loop {
        let mut name = String::new();
        let mut age_input = String::new();

        // Get name
        println!("Enter your name:");
        io::stdin().read_line(&mut name).expect("Failed");
        let name = name.trim();
        if name == "quit" { break; }

        // Get age
        println!("Enter your age:");
        io::stdin().read_line(&mut age_input).expect("Failed");

        let age: u32 = match age_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid age! Try again.\n");
                continue;
            }
        };

        let can_vote = age >= 18;
        let message = greet(name, age);
        println!("{}\n", message);

        log_voter(name, age, can_vote);
    }

    println!("Thank you, {}! Check voters.txt", "Naija");
}