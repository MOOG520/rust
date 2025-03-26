use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut secret = rand::thread_rng().gen_range(1, 101);

    let mut count: u32 = 3;

    loop{

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match  guess.trim().parse() {
            Ok(num) =>num,
            Err(_) => continue,
        };

        match guess.cmp(&secret){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number!");
                break;
            }
                
        }
        count = count - 1;
        if count == 0 {
            println!("Sorry, you ran out of guesses. The number was {}.", secret);
            break;
        }
    }

}