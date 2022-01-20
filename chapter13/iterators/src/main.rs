use crate::custom_iterator::Counter;

mod shoes;
mod custom_iterator;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


fn main() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    let v1_iter = v1.iter();
    println!("{:?}", v1);
    for val in v1_iter {
        println!("Got: {}", val);
    }
    // `for` loop takes the ownership over the iterator
    // println!("{:?}", v1_iter);
    println!("{:?}", v1);
    println!("{:?}", v2);

    for c in Counter::new() {
        println!("{}", c);
    }

    let mut x = &mut [1,2,3];
    for a in x.iter_mut() {
        *a += 1;
    }
    println!("{:?}", x);
}
