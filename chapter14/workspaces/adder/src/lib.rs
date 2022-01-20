use add_one;
use rand;
use rand::Rng;

pub fn main() {
    let num = rand::thread_rng().gen();
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}