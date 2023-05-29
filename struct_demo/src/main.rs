struct User {
    username: String,
    email: String,
    sign_in_count: usize,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 30,
        active: false,
    }
}

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("acb@126.com"),
        username: String::from("Nikky"),
        sign_in_count: 556,
        active: true,
    };

    // Tuple
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 长方形宽高
    calc_rect();

    // let params = parse_url("http://www.baidu.com?a=1");
    // println!("{:?}", params);
}

// 长方形宽高
// derive 派生 trait
#[derive(Debug)]
struct Rectangle {
    width: i32,
    length: i32,
}

// 使用元组定时宽高
fn area_tuple(dmi: (i32, i32)) -> i32 {
    dmi.0 * dmi.1
}
// 使用struct定义宽高
fn area(rect: &Rectangle) -> i32 {
    rect.width * rect.length
}

// impl for Rectangle
impl Rectangle {
    // 方法
    fn area(&self) -> i32 {
        self.width * self.length
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

// impl for Rectangle
impl Rectangle {
    // 关联函数 不把self作为第一个参数
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn calc_rect() {
    let rect_tuple = (10, 20);
    println!("使用tuple定义宽高 {}", area_tuple(rect_tuple));

    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("使用struct &rect 借用 {}", area(&rect));
    // :? 开启格式化
    println!("area(rect) 这样使用rect没有所有权 {:#?} \r\n", rect);

    println!(
        "impl 实现Rectangle 方法，实例直接调方法rect.area() {}",
        rect.area()
    );

    // can_hold
    let rect2 = Rectangle {
        width: 10,
        length: 10,
    };
    let rect3 = Rectangle {
        width: 30,
        length: 60,
    };
    println!("一个矩形能包含另一个矩形{}", rect.can_hold(&rect2));
    println!("一个矩形能包含另一个矩形{}", rect.can_hold(&rect3));

    // 关联函数
    let s = Rectangle::square(20);
    println!("关联函数值 {:#?}", s)
}

// 解析URL地址参数
fn parse_url(url: &str) -> Option<(String, String)> {
    let mut url_split = url.split("?");
    // let url_path = url_split.next()?;
    let url_query = url_split.next()?;
    let url_query_split = url_query.split("&");
    let mut url_query_kv = Vec::new();
    for query in url_query_split {
        url_query_kv.push(query);
    }
    let mut url_kv = Vec::new();
    for kv in url_query_kv {
        let mut kv_split = kv.split("=");
        let key = kv_split.next()?;
        let value = kv_split.next()?;
        url_kv.push((key, value));
    }
    println!("{:?}", url_kv);
    Some((url_kv[0].0.to_string(), url_kv[0].1.to_string()))
}
