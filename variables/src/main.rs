fn main() {
    let count = febonici(3);
    println!("End count = {count}");

    // first_word
    let mut str = String::from("hello world");
    let str_slice = &mut str[..5];
    let word = first_word(str_slice);
    // str.clear();
    println!("word = {word}");

    test_user_struct();
}
// 生成斐波那契数列
fn febonici(n: u32) -> u32 {
    // match n {
    //     0 => 0,
    //     1 => 1,
    //     _ => febonici( n - 1) + febonici(n - 2)
    // }
    if n < 1 {
        return 0;
    }
    let mut one = 1;
    let mut two = 0;
    let mut three = 0;
    for _i in 1..=n {
        three = one + two;
        one = two;
        two = three;
    }
    three
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn test_user_struct() {
    let user1 = build_user(String::from("user1@gmail.com"), String::from("user1"));
    // let user2 = build_user(user1.email, user1.username);
    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@gamil.com"),
        ..user1
    };
    println!("user1 = {user1:#?} user2 = {user2:#?}")
}
