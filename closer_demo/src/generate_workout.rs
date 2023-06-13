use std::{thread, time::Duration};

/**
 * 生成自定义运动计划
 * 目标：不让用户发生不必要等待
 * - 仅在必要时调用改算法
 * - 只调用一次
 */

pub fn run() {
    generate_workout(2, 4);
}

// 缓存器
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let y = (self.calculation)(arg);
                self.value = Some(y);
                v
            }
        }
    }
}

fn simulated_expensive_calculation(intersity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intersity
}

fn generate_workout(intersity: u32, random_number: u32) {
    let mut expensive_closer = Cacher::new(|intersity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intersity
    });
    // let expensive_result = simulated_expensive_calculation(intersity);
    if intersity < 25 {
        // 运动计划
        println!("Today, do {} pushups", expensive_closer.value(intersity));
        println!("Next, do {} situps", expensive_closer.value(intersity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minites",
                expensive_closer.value(intersity)
            );
        }
    }
}
