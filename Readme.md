## 2.1 猜數游戲

```rust
use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("猜數游戲");

    let secret_number = rand::thread_rng().gen_range(0..10);
    println!("神秘數字是{}", secret_number);

    loop {
        println!("猜測一個數");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading");
        println!("你猜测的数是{}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

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

```

## 3.1 变量与可变性

```rust
const MAX_POINT: i32 = 100_000;

fn main() {
    println!("Hello, world!");

    // variable immutable
    // ^^^^^ cannot assign twice to immutable variable
    let x = 5;
    println!("the value is {}", x);
    // mutable shadowing
    let mut x = x + 1;
    println!("shadowing the new value is {}", x);
    x = x * 2;
    println!("mut the new value is {}", x);
    // constant
    println!("const is {}", MAX_POINT);
    // 函数体表达式
    let y = {
        let a = 1;
        a + 1
    };
    println!("表达式块 is {}", y);

    // shadowing 改變變量類型
    let space = "   ";
    let space = space.len();
    println!("shadowing {}", space);

    // 函数返回值
    let v = five();
    println!("函數返回值 {}", v);
}

fn five() -> i32 {
    5
}


```

## 3.2 数据类型：标量类型

-   Rust 有四個標量類型
    -   整数类型
    -   浮点类型
    -   布尔类型
    -   字符类型

## 3.6 循环

```rust
fn main() {
    loop_demo();
    while_demo();
    for_demo();
    for_demo_lift();
}

fn loop_demo() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {}", result);
}

fn while_demo() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("lift out");
}

fn for_demo() {
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("for is {}", element);
    }
}

fn for_demo_lift() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("lift out");
}

```

## 4 所有权

### 4.1 什么是所有权
