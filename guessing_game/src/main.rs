use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    loop {
        println!("Guess a number");
        println!("Input your guess:");
    
        let mut guess = String::new();
        let output = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you didnt insert a valid number");
                continue;
            }
        };
    
        println!("You guessed {guess}");
        println!("{output}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too little"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
