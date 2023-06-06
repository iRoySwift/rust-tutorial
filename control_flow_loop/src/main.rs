use std::io;

fn main() {
    // loop_demo();
    // while_demo();
    // for_demo();
    // for_demo_lift();
    // break_continue_loop();
    // temperatures();
    println!("fibonacci: {}", fibonacci(9))
}

fn loop_demo() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);
}

fn while_demo() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("lift off!");
}

fn for_demo() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("for is: {}", element);
    }
}

fn for_demo_lift() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("lift off!");
}

fn break_continue_loop() {
    let mut count = 0;
    'continue_up: loop {
        let mut remaining = 10;
        println!("count = {}", count);
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'continue_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("the end count = {}", count);
}

/**
 * 相互转换摄氏与华氏温度
 */
fn temperatures() {
    println!("请选择：1.摄氏温度转换为华氏温度，2.华氏温度转换为摄氏温度");
    let mut flag: String = String::new();
    io::stdin()
        .read_line(&mut flag)
        .expect("Fail to read from stdin");
    let flag: u32 = flag.trim().parse().expect("not a number");

    if flag == 1 {
        println!("请输入摄氏温度：");
        let mut celsis: String = String::new();
        io::stdin()
            .read_line(&mut celsis)
            .expect("failed to read from stdin");
        let celsis: f32 = celsis.trim().parse().expect("not a number");
        let fahrenheit = celsis * 1.8 + 32.0;
        println!("{celsis}摄氏温度 = {fahrenheit}华氏温度");
    } else {
        println!("请输入华氏温度：");
        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("faild to read");
        let fahrenheit: f32 = fahrenheit.trim().parse().expect("not a  number");
        let celsis = (fahrenheit - 32.0) / 1.8;
        println!("{fahrenheit}华氏温度 = {celsis}摄氏温度");
    }
}

// 生成 n 阶斐波那契数列
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
