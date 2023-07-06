mod custom_sp;
mod deref_demo;
mod rc_sp;
mod refcell_messager;
mod refcell_rc_cons;
mod refcell_cons;
mod refcell_weak_cons;
mod refcell_weak_node;

fn main() {
    deref_demo::run();
    custom_sp::run();
    rc_sp::run();
    refcell_cons::run();
    refcell_rc_cons::run();
    refcell_weak_node::run();
}
