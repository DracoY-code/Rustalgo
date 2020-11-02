#[cfg(test)]
mod tests {
    use crate::recursive;

    #[test]
    fn factorial_check() {
        assert_eq!(recursive::factorial::get(4), 24);
    }

    #[test]
    fn fibonacci_check() {
        assert_eq!(recursive::fibonacci::get(5), 5);
    }
}

pub mod factorial {
    pub fn get(n: u32) -> u32 {
        match n {
            0 => 1,
            _ => n * get(n - 1)
        }
    }
}

pub mod fibonacci {
    pub fn get(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => get(n - 1) + get(n - 2)
        }
    }
}
