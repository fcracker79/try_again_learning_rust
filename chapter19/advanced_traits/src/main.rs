mod advanced_types;
use std::fmt;
use std::fmt::{Formatter, Pointer};

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Counter(max={}, current={}", self.max, self.current)
    }
}

impl OutlinePrint for Counter {

}


pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait MyGenericIterator<T=u32> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {
    current: u32,
    max: u32
}

impl Counter {
    pub fn new(max: u32) -> Counter {
        Counter {max: max, current: 0}
    }
}

impl MyGenericIterator for Counter {
    fn next(&mut self) -> Option<u32> {
        println!("Generic");
         if self.current >= self.max {
            return None;
        }
        self.current += 1;
        Some(self.current)
    }
}


impl MyIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        println!("Associated type");
        if self.current >= self.max {
            return None;
        }
        self.current += 1;
        Some(self.current)
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    println!("Hello, world!");
    let mut c = Counter::new(10);
    dbg!(MyIterator::next(&mut c));
    dbg!(MyGenericIterator::next(&mut c));

    let person = Human;
    // We do not need to be explicit as it refers to the bare `impl` for the struct
    person.fly();

    c.outline_print();

    advanced_types::main();
}
