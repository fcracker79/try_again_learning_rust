mod boxes;
mod derefs;
mod drop;
mod rc;
mod refcell;


fn main() {
    boxes::main();
    derefs::main();
    drop::main();
    refcell::main();
}