//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
pub mod public;
mod kinds;
mod utils;
/// Adds one to the number given. <br><h1 onclick="alert('hello')">HELLO</h1>
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, add_one(5));
    }
}
