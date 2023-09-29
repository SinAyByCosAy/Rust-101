use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number !");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_num}");

    loop{
        println!("Please input your guess");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num){
            //cmp compares and returns a variant of "Ordering" type (Less, Greater or Equal)
            //match is like a switch case(for pattern matching) that on receiving the variant goes
            //and matches against the keys or arms in Rust and executes the matching one
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
