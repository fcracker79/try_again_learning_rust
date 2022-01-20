use crate::boxes::List::{Cons, Nil};

enum List<T> {
    Cons(i32, Box<T>),
    Nil,
}

pub fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3, Box::new(Nil as List<i32>)))
                    ))
    );
    let b = String::from("hello");
    let b2 = Box::new(b);
    // Object moved into Box
    // println!("{}", b);
}