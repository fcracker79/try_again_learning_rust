fn dereference_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 = 7;
        println!("Dereference r1: {}", *r1);
    }
    let address = 0x0usize;
    let r = address as *const i32;
    unsafe {
        // SIGSEGV
        // println!("Dereference 0x12345: {}", *r);
    }

}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn use_extern_functions() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// This is to avoid function name mangling
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn main() {
    dereference_pointers();
    use_extern_functions();
}
