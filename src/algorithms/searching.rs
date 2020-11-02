#[cfg(test)]
mod tests {
    use crate::searching::*;
    use crate::utils::randomizer;
    
    #[test]
    fn search_check() {
        let mut arr: [u32; 10] = [0; 10];
        randomizer::rand_arr(&mut arr);

        let search: u32 = randomizer::gen_rand_num();
        
        let loc = jump::search(&mut arr, search);
        println!("{:?}", arr);

        if loc != -1 {
            println!("{} is present at {}", search, loc);
        } else {
            println!("{} is not found", search)
        }
    }
}

pub mod linear {
    pub fn search<T: PartialOrd>(arr: &[T], value: T) -> i32 {
        let n = arr.len();
        for i in 0..n {
            if arr[i] == value { return i as i32; }
        }
        -1
    }
}

pub mod binary {
    use crate::sorting::merge;

    pub fn search<T>(arr: &mut [T], value: T) -> i32
    where T:
        PartialOrd + Ord + Copy {
        if arr.is_empty() { return -1; }

        merge::sort(arr);

        let mut left = 0;
        let mut right = arr.len() - 1;

        while left < right {
            let mid = (left + right) / 2;

            if arr[mid] == value {
                return mid as i32;
            } else if arr[mid] < value {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

pub mod jump {
    use crate::sorting::merge;

    pub fn search<T>(arr: &mut [T], value: T) -> i32
    where T:
        PartialOrd + Ord + Copy {
        let n = arr.len();
        let mut m = (n as f64).sqrt() as usize;

        merge::sort(arr);

        while arr[m] <= value && m < n {
            m += (n as f64).sqrt() as usize;
            if m > n-1 { return -1; }
        }

        for j in 0..m {
            if arr[j] == value { return j as i32; }
        }

        -1
    }
}
