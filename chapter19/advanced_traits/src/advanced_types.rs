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

    returns_closure(4)(123);
    returns_closure_impl(3)(123);
}

// Returned function is accessed via vtable
fn returns_closure(i: u32) -> Box<dyn Fn(i32) -> i32> {
    let f1 = |x: i32| x + 1;
    let f2 = |x: i32| x + 2;
    if i % 2 == 0 {
        return Box::new(f1);
    }

    // Here I can return a different type as it uses vtables to determine the real implementation
    Box::new(f2)
}

// Function chooses the return value
fn returns_closure_impl(i: u32) -> impl Fn(i32) -> i32 {
    let f1 = |x: i32| x + 1;
    let f2 = |x: i32| x + 2;
    if i % 2 == 0 {
        return f1;
    }

    // Cannot return f1 as the `impl` forces the callee to decide the real instance.
    // `f1` is different from `f2` in terms of concrete types though...
    f1
}

pub fn main() {
    bang();
    let x: &str = "hello";
    // generic(x);
    generic_maybe_sized(x);
    // generic_not_sized(x);

    play_with_functions();
}