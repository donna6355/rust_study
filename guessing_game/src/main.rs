use std::io;
use rand::Rng;
use std::cmp::Ordering;

//create type to handle error
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value:i32) -> Guess {
        if value<1 || value>100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Guess {value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}",secret_number);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if(guess < 1 || guess > 100 ){
            println!("The secret number will be between 1 and 100");
            continue;
        }
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}
