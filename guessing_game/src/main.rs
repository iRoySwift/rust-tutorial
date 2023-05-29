use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("猜數游戲");

    let secret_number: i32 = rand::thread_rng().gen_range(0..10);
    println!("神秘數字是{}", secret_number);

    loop {
        println!("猜測一個數");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading");
        println!("你猜测的数是{}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        // if guess < 1 || guess > 100 {
        //     println!("Guess value must be between 1 and 100, got {}", guess);
        //     continue;
        // }

        let guess: Guess = Guess::new(guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// #[derive(Iterator)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value)
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
