use guess_game;
mod common;

// 只运行 某个测试方法
// cargo test -p guess_game greater_than_100
// cargo test 的 --test 后跟文件的名称
// cargo test -p guess_game --test guess_new
#[test]
fn less_than_100() {
    common::setup();
    guess_game::Guess::new(50);
}
