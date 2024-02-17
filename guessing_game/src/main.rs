use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
pub struct Guess {
    val: u8,
}

impl Guess {
    pub fn new(val: u8) -> Guess {
        if val < 1 || val > 10 {
            panic!("Number greater than specified range of the type, got {val}")
        }
        Guess { val }
    }

    pub fn get(&self) -> u8 {
        self.val
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess.(1,10)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        println!("You guessed: {}", guess.get());

        match guess.get().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
