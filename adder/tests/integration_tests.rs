// use adder;

mod common;

#[test]
fn it_works_result() -> Result<(), String> {
    common::setup();
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two doesn't equal four"))
    }
}
