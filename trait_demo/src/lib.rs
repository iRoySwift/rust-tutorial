use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
    // 默认实现
    // fn summarize(&self) -> String {
    //     String::from("(Read more...)")
    // }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{},{} {},by", self.headline, self.location, self.author)
    }
}

// impl trait
pub fn notices1(item: impl Summary + Display) {
    println!("Breaking news! {:?}", item.summarize());
}

// trait bound 使用复杂情况
pub fn notices2<T: Summary + Display>(item: &T) {
    println!("Breaking news! {:?}", item.summarize());
}

// 使用 + 指定多个trait bound
pub fn notices3<T: Summary + Display, U: Copy + Debug>(a: T, b: U) -> String {
    format!("Breaking news! {:?}", a.summarize())
}

// trait bound 在方法签名后指定where子句
pub fn notices<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Copy + Debug,
{
    format!("Breaking news! {:?}, {:?}", a.summarize(), b)
}

// pub fn returns_summarizable(flag: bool) -> impl Summary {
//     if flag {
//         NewArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 r#"The Pittsburgh Penguins once again are the best hockey team in the NHL."#,
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebook"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
