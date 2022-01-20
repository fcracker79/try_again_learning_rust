use std::mem::drop;

// #[derive(Debug, Copy, Clone)]
//                 ^^^^ Copy not allowed on types with destructors
struct CustomSmartPointer<'a> {
    data: &'a str,
}


impl<'a> Drop for CustomSmartPointer<'a> {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


pub fn main() {
    println!("Playing with drop");
    {
        let c = CustomSmartPointer {
            data: "enclosed scope stuff",
        };
    }
    let c = CustomSmartPointer {
        data: "my stuff",
    };
    let d = CustomSmartPointer {
        data: "other stuff",
    };
    println!("CustomSmartPointers created.");
    let c = CustomSmartPointer {
        data: "enclosed scope stuff",
    };
    drop(c);
    // println!("{:?}", c); // value moved, cannot use again
}