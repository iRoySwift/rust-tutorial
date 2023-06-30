mod do_panic;

fn main() {
    println!("Hello, world!");
    do_panic::match_open_txt();
    do_panic::unwrap_open_txt();
    do_panic::unwrap_expect_text();
    let _result = do_panic::read_username_from_file();
}
