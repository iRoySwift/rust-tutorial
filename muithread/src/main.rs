mod channel;
mod move_thread;
mod mutex_lock;
mod thread_main;

fn main() {
    thread_main::run();
    move_thread::run();
    channel::run();
    mutex_lock::run();
}
