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
