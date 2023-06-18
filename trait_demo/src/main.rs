use trait_demo::{Summary, Tweet};
mod longest;

fn main() {
    // 获取最大值
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Hello, world!,{}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result_char = largest(&char_list);
    println!("{}", result_char);

    let str_list = vec![String::from("Hello"), String::from("word")];
    let result_str = largest(&str_list);
    println!("{}", result_str);
    summary_test();

    // 字符串最大长度值
    longest::longest_test();
}

// 范型
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 为类型实现trait
fn summary_test() {
    let twitter = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    // notices(returns_summarizable());
    println!("{}", twitter.summarize())
}
