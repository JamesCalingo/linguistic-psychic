use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..1001);
    let mut tries = 0;
    // println!("The secret number is {}", secret_number);
    loop {
        println!("Input guess now.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                tries += 1;
                println!("Higher...{}", tries)},
            Ordering::Greater => {
                tries += 1;
                println!("Lower...{}", tries)},
            Ordering::Equal => {
                tries += 1;
                println!("That's it!");
                println!("Guesses: {}", tries);
                break;
            }
        }
    }
}
