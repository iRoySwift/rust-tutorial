use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub mod guess {
    use super::*;

    pub fn run() {
        println!("猜数游戏！");
        let secret_number: i32 = rand::thread_rng().gen_range(0..=100);
        println!("神秘数字是:{}", secret_number);
        loop {
            println!("请输入一个0~100之间的数字:");

            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("输入有误！");
            let guess: i32 = match guess.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    continue;
                }
            };
            println!("你猜的数字是：{:?}", guess);

            let guess = Guess::new(guess);

            match guess.value().cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 || value > 100 {
            panic!("猜的数字必须在0-100之间! {}", value);
        }
        Guess { value }
    }
    fn value(&self) -> i32 {
        self.value
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    // cargo test -p guess_game greater_than_100
    #[test]
    #[ignore]
    #[should_panic(expected = "猜的数字必须在0-100之间")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
