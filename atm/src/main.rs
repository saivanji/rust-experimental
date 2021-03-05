use std::io;
use std::process;

const CORRECT_PIN: &str = "0000";

fn main() {
    let is_allowed = ask_pin(3);
    let balance = 4000;

    if !is_allowed {
        process::exit(0);
    }

    println!("Your balance is {}", balance)
}

fn ask_pin(attempts: u8) -> bool {
    println!("Enter pin code");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Pin read failure");

    let is_correct = input.chars().count() == 5 && &input[0..4] == CORRECT_PIN;

    if !is_correct && attempts == 1 {
        println!("Pin code incorrect. You have no more attempts. Good bye.");
        return false;
    }

    if !is_correct {
        println!("Pin code incorrect. Try again.");
        return ask_pin(attempts - 1);
    }

    true
}
