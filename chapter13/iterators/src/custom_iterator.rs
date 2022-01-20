pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 5 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Counter;

    #[test]
    fn test() {
        let expected: Vec<(u32, u32)> = vec!( (4, 5), (3, 4), (2, 3), (1, 2), (0, 1));
        let got: Vec<(u32, u32)> = Counter::new().zip(Counter::new().map(|x| x + 1)).collect();
        assert_eq!(
            expected,
            got
        )
    }
}