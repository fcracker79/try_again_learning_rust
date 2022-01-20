use std::rc::Rc;


enum List {
    Cons(String, Rc<List>),
    Nil,
}

use crate::rc::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(String::from("5"), Rc::new(Cons(String::from("10"), Rc::new(Nil)))));
    // The standard `clone` method would have deep-copied the object.
    // `Rc::clone`, instead, just increases the reference counter.
    // More importantly, in order to invoke `clone`, you should have the inner type implementing `Clone` as well.
    // That is not a requirement for `Rc::clone`, as it does not need to.
    let b = Cons(String::from("3"), Rc::clone(&a));
    let c = Cons(String::from("4"), Rc::clone(&a));
}