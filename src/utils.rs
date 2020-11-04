#[cfg(test)]
mod tests {
    use crate::utils::randomizer;

    #[test]
    fn random_number_check() {
        println!("{}", randomizer::gen_rand_num());
    }

    #[test]
    fn array_randomizer_check() {
        let mut arr: [u32; 10] = [0; 10];
        randomizer::rand_arr(&mut arr);
        println!("{:?}", arr);
    }
}

pub mod randomizer {
    use rand::Rng;

    pub fn gen_rand_num() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0, 101)
    }

    pub fn rand_arr(list: &mut [u32]) {
        let n = list.len();
        
        for i in 0..n {
            list[i] = gen_rand_num();
        }
    }
}

use std::{io::*, fmt};

pub fn input(message: &'_ impl fmt::Display) -> String {
    print!("{}", message);
    stdout().flush().expect("Error while flushing to stdout");
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Error while reading stdin");
    line
}
