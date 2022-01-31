fn simple_if_let() {
    let favorite_color: Option<&str> = Some("Red");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(favorite_color) = favorite_color {
        println!("Using your favorite color, {}, as the background", favorite_color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn simple_while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn simple_for() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn playing_with_iterators() {
    let array = [0; 3];
    for x in array.into_iter() {
        // _x += 1;
    }

    let array = &mut [0; 3];
    for x in array.into_iter() {
       *x += 1
    }

    let array = &[0; 3];
    for x in array.into_iter() {
        // _x += 1;
    }
    dbg!(array);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}


fn play_with_function_pattern_matching() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn play_with_option_pattern() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // None not covered
    // let Some(x) = Some(10);
    if let Some(x) = Some(10) {
        println!("Yeah!");
        dbg!(x);
    } else {
        println!("Nope");
    }
}

fn play_with_range_patterns() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn play_with_structs() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    println!("Pattern matching over structs works");

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn play_with_underscore() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn play_with_enum_patterns() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}

fn play_with_ignored() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 1, y: 2, z: 3 };

    match origin {
        Point { x, z, .. } => println!("x is {}, z is {}", x, z),
    }
}

fn play_with_conditional_guards() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    match x {
        // Pity, does not work
        // 4 | 5 | (6 if y) => println!("yes"),
        4 | 5 => println!("yes"),
        6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn play_with_at() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
fn main() {
    simple_if_let();
    simple_while_let();
    playing_with_iterators();
    play_with_function_pattern_matching();
    play_with_option_pattern();
    play_with_range_patterns();
    play_with_structs();
    play_with_enum_patterns();
    play_with_underscore();
    play_with_ignored();
    play_with_at();
}
