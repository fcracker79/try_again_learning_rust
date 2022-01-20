#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub fn this_function_will_panic() {
    panic!("Panic! Well, I don't know why...");
}


#[cfg(test)]
mod testmodule {
    use super::*;

    #[test]
    fn another_test() -> Result<(), String> {
        Ok(())
    }
}