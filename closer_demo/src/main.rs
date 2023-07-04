mod fn_immut;
mod fn_mut;
mod fn_once;
mod generate_workout;

fn main() {
    println!("Hello, world!");
    generate_workout::run();
    fn_once::run();
    fn_mut::run();
    fn_immut::run();
}
