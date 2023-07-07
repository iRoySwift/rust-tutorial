// use oop::{Button, Draw, Screen};

use oop::blog;
use oop::gui_draw::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {}
}

fn main() {
    println!("Hello, world!");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("Hello, world!"), String::from("World!")],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Hello, world!"),
            }),
        ],
    };
    screen.run();
    blog::run();
}
