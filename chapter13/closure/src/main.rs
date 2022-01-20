use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}


impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn f_once<T: FnOnce(u32) -> String>(ff: T) {
    let a: u32 = 1;
    ff(a);
    // this cannot be done again, as ff is an FnOnce
    // ff(a);

}

fn f_mut<T: FnMut(u32) -> String>(mut ff: T) {
    let a: u32 = 1;
    ff(a);
    ff(a);
}

fn f_normal<T: Fn(u32) -> String>(ff: T) {
    let a: u32 = 1;
    ff(a);
    ff(a);
}

fn test_fn() {
    let d = String::from("hello");
    // Fn
    let ff = |a| { println!("{}", d); String::from("unrelevant")};
    f_once(ff);
    f_mut(ff);
    f_normal(ff);
    println!("{}", d);
}

fn test_fn_mut() {
    let mut d2 = String::from("hello");
    // FnMut
    let fnmut = |a: u32| { d2 += " world"; println!("{}", d2); String::from("unrelevant")};

    // fnmut would result moved and thus I cannot pass it to `f_mut`
    // f_once(fnmut);
    f_mut(fnmut);
    // This does not work because fnmut is an FnMut
    // f_normal(fnmut);
}

fn a_function_that_moves_string(s: String) -> String {
    String::from("")
}

fn test_fn_once() {
    let d2 = String::from("hello");
    // FnOnce
    let fnonce = |a: u32| d2;

    f_once(fnonce);
    // These will not work as fnonce is an FnOnce
    // f_mut(fnonce);
    // f_normal(fnonce);
    let d2 = String::from("hello");
    let fnonce = |a: u32| a_function_that_moves_string(d2);
    f_once(fnonce);
    // These will not work as fnonce is an FnOnce
    // f_mut(fnonce);
    // f_normal(fnonce);

    let d2 = String::from("hello");
    let fnonce = move |a: u32| String::from("x") + &d2;
    // ??? Shouldn't `move` create an FnOnce instance?
    f_normal(fnonce);
    // d2 IS moved but it is used as a read-only reference and this seems to be enough to get an Fn
    // println!("{}", d2);

    let d2 = String::from("hello");
    let fnonce = ensure_fn_once(|a: u32| String::from("x") + &d2);
    // f_normal(fnonce);
    f_once(fnonce);

    let d2 = String::from("hello");
    let fnonce = ensure_fn_once_generics(|a: u32| String::from("x") + &d2);
    //f_normal(fnonce);
    f_once(fnonce);

}

trait MyFoo {}

impl MyFoo for u32 {}

fn foo<T: MyFoo>(t: T) {}
fn bar(t: impl MyFoo) {}

fn test_quantifiers() {
    foo::<u32>(0); // this is allowed
    // Explicit generic argument not allowed
    // bar::<u32>(0); // this is not
    bar(0);
    let ff1 = foo::<u32>;
    //let ff = bar::<u32>;
}

fn ensure_fn_once(f: impl Fn(u32) -> String) -> impl FnOnce(u32) -> String {
    f
}

fn ensure_fn_once_generics<T: Fn(u32) -> String>(f: T) -> impl FnOnce(u32) -> String {
    f
}

fn main() {
    generate_workout(20, 0);
    test_fn();
    test_fn_mut();
    test_fn_once();
}


#[cfg(test)]
mod test {
    use crate::Cacher;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}