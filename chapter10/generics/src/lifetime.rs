/*
// lifetimes 'b < 'a, so I cannot use stuff from 'b in 'a
fn boh() {
        let r;          // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;   // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
*/

use std::fmt;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
        // BEWARE
        // returns a reference to data owned by the current function
        // String::from("ciao").as_str()
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // third lifetime elision rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    fn announce_announcement<'b>(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        announcement
    }
}

// Elision
fn create_important_excerpt(x: &str) -> ImportantExcerpt {
    ImportantExcerpt {
        part: x
    }
}
// fn create_important_excerpt<'a>() -> ImportantExcerpt<'a> {
//     ImportantExcerpt {
//         part: String::from("hello").as_str()
//               --------------------- temporary value created here
//     }
//     ^ returns a value referencing data owned by the current function
// }

impl<'a> fmt::Display for ImportantExcerpt<'a> {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImportantExcerpt({})", self.part)
    }
}

fn struct_stuff(novel: &str) -> ImportantExcerpt {
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    i
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// By default T is Sized
struct Foo<T>(T);
struct Bar<T: ?Sized>(T);

// struct FooUse(Foo<[i32]>); // error: Sized is not implemented for [i32]
struct BarUse(Bar<[i32]>);


pub fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    println!("Important excerpt: {}", struct_stuff(novel.as_str()));
}