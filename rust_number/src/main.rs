use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! It's between 1 and 1000 if that helps.");

    let secret_number = rand::thread_rng().gen_range(1..1001);
    let mut tries = 0;
    // println!("The secret number is {}", secret_number);
    loop {
        tries += 1;
        println!("Your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > 1000 {
            println!("The secret number is less than 1000.");
            continue
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("The secret number is higher than {}", guess)
            }
            Ordering::Greater => {
                println!("The secret number is lower than {}", guess)
            }
            Ordering::Equal => {
                if tries == 1 {
                    println!("Wow! You got that in one try!");
                }else {
                println!("You got it in {} guesses!", tries);
                }
                break;
            }
        }
    }
}
