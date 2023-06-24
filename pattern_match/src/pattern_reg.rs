// let for 只可使用无可辩驳模式
// if let, while let 可以使用可辩驳模式  无可辩驳模式

// 匹配命名变量
pub fn named_var() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),

        // 命名变量是可匹配任何值的无可辩驳模式
        Some(y) => println!("Matched, y = {}", y),
        _ => println!("Default case: x = {:?}", x),
    }

    println!("at the end: x = {:?} , y = {:?}", x, y);
}

// 多重模式 |
fn match_or() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("default case"),
    }
}

// 使用 ..= 来匹配某个范围的值
fn match_dot() {
    let x = 1;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("default case"),
    }

    let x = 'a';
    match x {
        'a'..='j' => println!("early ascII letter"),
        'k'..='z' => println!("late ascII letter"),
        _ => println!("default case"),
    }
}

// 解构以分解值
// 可以使用模式来解构struct、enum、tuple, 从而引用这些类型值的不同部分
struct Point {
    x: usize,
    y: usize,
}
fn deconstruct_assignment() {
    let p = Point { x: 1, y: 2 };

    let Point { x, y } = p;
    assert_eq!(1, x);
    assert_eq!(2, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 1, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neighter axis at ({},{})", x, y),
    }
}

// 解构枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn deconstruct_enum() {
    let msg = Message::ChangeColor(0, 16, 255);

    match msg {
        Message::Quit => println!("The Quit"),
        Message::Move { x, y } => println!("move in the x direction {} and in the y derectioin", x),
        Message::Write(text) => println!("text is {}", text),
        Message::ChangeColor(r, g, b) => println!("{},{},{}", r, g, b),
    }
}

// 在模式中忽略值
// _
// _配合其它模式  Some(_)
// 使用以_开头的名称
// .. 忽略值的剩余布冯
fn ignore_value() {
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (1, 2, 3, 4, 5, 6, 7, 8);
    match numbers {
        (first, .., last) => {
            println!("Some numbers are {},{}", first, last)
        } // (.., sencond, ..) => {
          //     println!("sencond are {}", sencond)
          // }
    }
}

// 使用match守卫来提供额外的条件
fn match_if() {
    let num = Some(4);
    let x = 4;
    let y = 10;
    match num {
        Some(x) if x < 5 => println!("less than five:{}", x),
        Some(n) if n == y => println!("Match, n = {:?}", n),
        Some(x) => println!("{}", x),
        None => (),
    }

    match x {
        4 | 5 | 6 if y == 10 => println!("yes"),
        _ => println!("no"),
    }
}

// @绑定
enum Info {
    Hello { id: i32 },
}
fn bind_pattern() {
    let msg = Info::Hello { id: 5 };

    match msg {
        Info::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range:{}", id_variable)
        }
        Info::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Info::Hello { id } => {
            println!("Found other")
        }
    }
}
