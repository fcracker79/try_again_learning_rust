#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(ChangeColorMessage),
}


struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
    boh: u8
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) -> String {
        match self {
            Message::Quit => String::from("Quitting"),
            Message::Write(s) => format!("Writing \"{}\"", s),
            Message::Move {x, y} => format!("Moving to ({}, {})", x, y),
            Message::ChangeColor(ChangeColorMessage(r, g, b)) =>
                format!("Changing to color {}, {}, {}", r, g, b)
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Loopback: {:?}", loopback);
    println!("Home: {:?}", home);

    let m = Message::Write(String::from("hello"));
    println!("{}", m.call());
    let m = Message::Quit;
    println!("{}", m.call());
    let m = Message::Move {x: 1, y: 2};
    println!("{}", m.call());
    let m = Message::ChangeColor(ChangeColorMessage(1, 3, 5));
    println!("{}", m.call());
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some string: {:?}", some_string);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    };

}
