use std::collections::HashMap;

struct Boh {
    i: i32
}

impl Boh {
    fn get(&self) -> &i32 {
        &self.i
    }

    fn change(&mut self) {
        self.i = 32;
    }
}


fn vectors_of_enums() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn vectors() {
    fn whatever(x: Vec<i32>) {}
    fn whatever2(x: Vec<&str>) {}

    let v = Vec::new();
    let v2 = Vec::new();
    whatever(v);
    whatever2(v2);

    let v = vec![1, 2, 3];
    let y = v[1];
    // i32 is Copy, so I can refer to it directly.
    println!("{:?}, {:?}", v, y);
    let mut v = Vec::new();
    v.push(String::from("one"));
    v.push(String::from("two"));
    v.push(String::from("three"));
    v.push(String::from("four"));
    // cannot move out of index of `Vec<String>`
    // move occurs because value has type `String`, which does not implement the `Copy` trait
    // help: consider borrowing here: `&v[2]`
    // let third = v[2];
    let third = &v[2];
    let third = v.get(2);
    println!("{:?}, {:?}", v, third);

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // immutable borrow occurs here
    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // v.push(6); // mutable borrow occurs here
    // println!("The first element is: {}", first); // immutable borrow later used here
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = v.get(0);
    // v.push(6);
    // println!("The first element is: {:?}", first);

    // cannot borrow `x` as mutable because it is also borrowed as immutable
    // let mut x = Boh {i: 12345};
    // let y = x.get(); // immutable borrow occurs here
    // x.change();
    // println!("{}", y); // immutable borrow later used here

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v);
    for i in v {
        println!("{}", i);
    }
    // borrow of moved value: `v`
    // println!("{:?}", v);
}

fn strings() {
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo0000");
    let x = &s[0..4];
    s.push_str("bar");
    // cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("{}, {}", x, s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // println!("{} {} {}", s1, s2, s3);
    println!("{} {}", s2, s3);

    let hello = "Здравствуйте";
    println!("length of string \"{}\": {}", hello, hello.len());
    println!("{:x?}", hello.as_bytes());
    // З != 3
    let hello = "З";
    println!("length of string \"{}\": {}", hello, hello.len());
    println!("{:x?}", hello.as_bytes());
}

fn hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Hash Map {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Hash Map {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("{}", field_name);
    println!("{}", field_value);
    println!("{:?}", map);
    println!("{:?}", map.get(&"Favorite color".to_string()));


    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    field_name.push_str("ciao");
    // cannot borrow `field_name` as mutable because it is also borrowed as immutable
    // for (key, value) in &map {
    //     println!("{}: {}", key, value);
    // }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn main() {
    println!("Hello, world!");
    vectors();
    vectors_of_enums();
    strings();
    hashmaps();
}
