#[cfg(test)]
mod tests {
    use crate::sorting::*;
    use crate::utils::randomizer;

    #[test]
    fn sort_check() {
        let mut arr: [u32; 10] = [0; 10];
        randomizer::rand_arr(&mut arr);
        merge::sort(&mut arr);
        println!("{:?}", arr);
    }
}

pub mod bubble {
    pub fn sort<T: PartialOrd>(arr: &mut [T]) {
        let n = arr.len();
        let mut swapped: bool;
        
        for i in 0..n-1 {
            swapped = false;

            for j in 0..n-i-1 {
                if arr[j] > arr[j+1] {
                    arr.swap(j, j+1);
                    swapped = true;
                }
            }

            if swapped == false { break; }
        }
    }
}

pub mod insertion {
    pub fn sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        let n = arr.len();
        
        for i in 1..n {
            let key = arr[i];
            let mut j = i - 1;

            while arr[j] > key {
                arr[j+1] = arr[j];
                if j == 0 { break; }
                j -= 1;
            }
            
            arr[j] = key
        }
    }
}

pub mod selection {
    pub fn sort<T: PartialOrd>(arr: &mut [T]) {
        let n = arr.len();
        
        for i in 0..n-1 {
            let mut min = i;

            for j in i+1..n {
                if arr[j] < arr[min] {
                    min = j;
                }
            }

            if min != i {
                arr.swap(min, i);
            }
        }
    }
}

pub mod merge {
    pub fn sort<T: Copy + Ord>(arr: &mut [T]) {
        let n = arr.len();
        let m = n / 2;

        if n == 1 { return; }
        
        sort(&mut arr[0..m]);
        sort(&mut arr[m..n]);

        let mut y: Vec<T> = arr.to_vec();

        merge(&arr[0..m], &arr[m..n], &mut y[..]);

        arr.copy_from_slice(&y);
    }
    
    fn merge<T: PartialOrd + Copy>(x1: &[T], x2: &[T], y: &mut [T]) {
        assert_eq!(x1.len() + x2.len(), y.len());

        let (mut i, mut j, mut k) = (0, 0, 0);

        while i < x1.len() && j < x2.len() {
            if x1[i] < x2[j] {
                y[k] = x1[i];
                i += 1;
            } else {
                y[k] = x2[j];
                j += 1;
            }
            k += 1;
        }

        if i < x1.len() {
            y[k..].copy_from_slice(&x1[i..]);
        }
        if j < x2.len() {
            y[k..].copy_from_slice(&x2[j..]);
        }
    }
}

pub mod shell {
    pub fn sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        let n = arr.len();
        let mut h: usize = 1;

        while h <= n/3 {
            h = h*3 + 1;
        }

        while h > 0 {
            for i in h..n {
                let key = arr[i];
                let mut j = i;

                while j > h-1 && arr[j-h] >= key {
                    arr[j] = arr[j-h];
                    j -= h;
                }

                arr[j] = key;
            }

            h = (h - 1) / 3;
        }
    }
}

pub mod quick {
    pub fn sort<T: PartialOrd + Copy>(
        arr: &mut [T],
        low: usize,
        high: usize
    ) {
        if low >= high { return; }
        
        let p = partition(arr, low, high);

        sort(arr, low, (p-1) as usize);
        sort(arr, (p+1) as usize, high);
    }

    fn partition<T: PartialOrd + Copy>(
        arr: &mut [T],
        low: usize,
        high: usize
    ) -> isize {
        let mut i = low;
        let pivot = arr[high];

        for j in low..high {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
        }

        arr.swap(i, high);
        i as isize
    }
}
