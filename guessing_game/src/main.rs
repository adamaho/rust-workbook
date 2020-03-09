use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let answer = rand::thread_rng().gen_range(1, 101);
    
    loop {
        let mut guess = String::new();
        println!("Enter a guess");
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    
        match guess.cmp(&answer) {
            Ordering::Greater => println!("Guess is too high"),
            Ordering::Less => println!("Guess is too low"),
            Ordering::Equal => {
                println!("You are correct!");
                break;
            }
        }
    }

}