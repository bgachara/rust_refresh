pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess the value must the between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(12, 2);
        assert_eq!(result, 14);
    }

    #[test]
    fn itdoesnt_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn greater_than() {
        Guess::new(200);
    }
}

//assert_eq! and assert_ne! macros for equality test.
//how are tests run???
