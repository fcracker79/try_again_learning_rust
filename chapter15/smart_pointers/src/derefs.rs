use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn dino(x: &i32) {}

pub fn main() {
    let x = &5;
    println!("Deref using normal pointers");
    assert_eq!(5, *x);
    let x = Box::new(5);
    println!("Deref using boxes");
    assert_eq!(5, *x);

    let x = Box::new(Box::new(5));
    println!("Deref using recursive boxes");
    assert_eq!(5, **x);
    let x = Box::new(Box::new(Box::new(5)));
    assert_eq!(5, ***x);

    let x = MyBox::new(5);
    println!("Deref using myboxes");
    assert_eq!(5, *x);

    hello(&Box::new(String::from("world")));

    // Deref coercion is recursive
    let x = Box::new(Box::new(5));
    dino(&x);

    let x = Box::new(Box::new(String::from("hello")));
    let y = x.as_str();
}