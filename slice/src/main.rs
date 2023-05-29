fn main() {
    // 全部切片
    let s = "Hello, world!";

    let word_index = first_word(s);
    println!("{}", word_index);

    let hello_str = String::from("Hello, world!");
    // 切片
    let hello = &hello_str[..5];
    let world = &hello_str[6..];
    let whole = &hello_str[..];
    let find_word = first_word(&hello_str);
    // mutable borrow occurs here
    // hello_str.clear();
    println!("{} | {} | {} | {}", hello, world, whole, find_word);
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
