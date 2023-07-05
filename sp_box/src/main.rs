mod custom_sp;
mod deref_demo;
mod rc_refcell_list;
mod rc_sp;
mod rc_weak;
mod refcell;
mod refcell_rc;

fn main() {
    deref_demo::run();
    custom_sp::run();
    rc_sp::run();
    refcell_rc::run();
    rc_refcell_list::run();
    rc_weak::run();
}
