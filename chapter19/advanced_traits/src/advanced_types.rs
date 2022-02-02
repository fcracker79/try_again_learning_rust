fn no_value() -> ! {
    loop {}
}


fn bang() {
    let v = Some(123);
    let x = match v {
        Some(_) => 32,
        None => no_value()
    };
    println!("{}", x);
}

fn generic<T>(t: &T) {
    // --snip--
}

fn generic_maybe_sized<T: ?Sized>(t: &T) {
    // --snip--
}

// Negative bounds are not supported
// fn generic_not_sized<T: !Sized>(t: &T) {
//     // --snip--
// }

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice_trait<T: Fn(i32) -> i32>(f: T, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn play_with_functions() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    let answer = do_twice_trait(add_one, 5);
    println!("The answer is: {}", answer);

    do_twice(|x| x + 1, 1234);
    do_twice_trait(|x| x + 1, 1234);

    returns_closure()(123);
    returns_closure_impl()(123);
}

// Returned function is accessed via vtable
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// Function chooses the return value
fn returns_closure_impl() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

pub fn main() {
    bang();
    let x: &str = "hello";
    // generic(x);
    generic_maybe_sized(x);
    // generic_not_sized(x);

    play_with_functions();
}