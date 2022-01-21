mod boxes;
mod derefs;
mod drop;
mod rc;
mod refcell;
mod rc_and_refcell;
mod memory_leak;
mod tree;


fn main() {
    boxes::main();
    derefs::main();
    drop::main();
    refcell::main();
    rc_and_refcell::main();
    memory_leak::main();
    tree::main();
}