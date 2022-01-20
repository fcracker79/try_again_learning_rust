fn f1() {
    println!("f1");
    let s = String::from("hello");
    let s2 = s;
    // println!("{}", s);
    println!("{}", s2);
}

fn f2() {
    println!("f2");
    let s = String::from("hello");
    let s2 = s.clone();
    println!("{}", s);
    println!("{}", s2);
}

fn f3() {
    println!("f3");
    let s = 5;
    let s2 = s;
    println!("{}", s);
    println!("{}", s2);
}

fn f4() {
    println!("f4");
    let s = 5;
    let s2 = s.clone();
    println!("{}", s);
    println!("{}", s2);
}

fn f5sub(x: String) {
    println!("{}", x);
}

fn f5sub2(x: &String) {
    println!("{}", x);
}

fn f5() {
    println!("f5");
    let x = String::from("hello");
    f5sub(x);
    // Value borrowed here after move
    // println!("{}", x);
    let x = String::from("hello");
    f5sub2(&x);
    println!("{}", x);
}

fn f6sub(x: &mut String) {
    x.push_str(" world");
}

fn f6() {
    println!("f6");
    let mut x = String::from("hello");
    f6sub(&mut x);
    println!("{}", x);
}

fn f7() {
    println!("f7");
    let mut x = String::from("hello");
    let r1 = &x;
    let r2 = &x;
    println!("{}", r1);
    println!("{}", r2);
    let rm = &mut x;
    println!("{}", rm);
    // cannot borrow `x` as mutable because it is also borrowed as immutable
    // println!("{} {} {}", r1, r2, rm);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn f8() {
    println!("f8");
    let s = String::from("hello world");
    println!("{}", first_word(&s));
}

fn f9() {
    println!("f9");
    let mut s = String::from("hello world");
    let f = first_word(&s);
    // mutable borrow occurs here
    // s.clear();
    // immutable borrow later used here
    println!("{}", f);
    s.clear();
}

fn main() {
    f1();
    f2();
    f3();
    f4();
    f5();
    f6();
    f7();
    f8();
    f9();
}
