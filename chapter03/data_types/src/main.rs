use std::cmp::Ordering;

fn f() -> u8 {
    5
}

fn f_tuple(t: (u8, &str)) {
    let (a, b) = t;
    println!("({}, {})", a, b);
}

fn main() {
    let poo = '💩';
    println!("Hello, world! {}", poo);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let (x0, y0, z0) = x;
    let x00 = x.0;
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("For element in a: {}", element);
    }

    let mut i = 0;
    while i < 5 {
        println!("While condition {}", a[i]);
        i += 1;
    }

    i = 0;
    'out: loop {
        println!("Loop {}", a[i]);
        i += 1;
        if i >= 5 {
            break 'out;
        }
    }

    let first = a[0];
    let second = a[1];
    let b = [5; 30];
    println!("{} - {} - {}", b.len(), b[0], b[b.len() -1]);
    //let xx = f();
    let xx = 5;
    let yy: u32 = 32;
    /*
    Hello
     */
    match xx.cmp(&yy) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    f_tuple((1, "hello"));
}
