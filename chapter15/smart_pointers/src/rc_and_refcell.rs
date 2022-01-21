#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<RefCell<List>>),
    Nil,
}

use std::cell::RefCell;
use std::rc::Rc;
use crate::rc_and_refcell::List::{Cons, Nil};

fn create_ref<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}

fn order_matters() {
    println!("Showing difference between Rc<Refcell<T>> and RefCell<Rc<T>>");
    let a = Rc::new(1);
    let b = Rc::new(2);

    let c = RefCell::new(Rc::clone(&a));
    let d = RefCell::new(Rc::clone(&a));
    println!("RefCell<Rc<T>>");
    println!("Before change:\nc:{:?}\nd:{:?}", c, d);
    *d.borrow_mut() = Rc::clone(&b); // this doesn't affect c
    println!("After change:\nc:{:?}\nd:{:?}", c, d);

    let a = RefCell::new(1);
    let b = RefCell::new(2);
    let c = Rc::new((a));
    let d = Rc::clone(&c);
    println!("Rc<RefCell<T>>");
    println!("Before change:\nc:{:?}\nd:{:?}", c, d);
    *c.borrow_mut() = 123;
    println!("After change:\nc:{:?}\nd:{:?}", c, d);
}

pub fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = create_ref(Cons(Rc::clone(&value), create_ref(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let value = Rc::new(RefCell::new(12345));
    *a.borrow_mut() = Cons(Rc::clone(&value), create_ref(Nil));
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    order_matters();
}