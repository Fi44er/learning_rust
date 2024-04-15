#![warn(clippy::all, clippy::pedantic)]

use std::cmp::Ordering;

fn main() {
    let arr: [i32; 10] = [-1, 2, 3, 10, 50, 100, 130, 132, 150, 160];
    let result = bin_search(&arr, 3);
    match result {
        Some((value, index)) => println!("Found {value} at index {index}"),
        None => println!("Not found"),
    }
}

fn bin_search(arr: &[i32], disired_value: i32) -> Option<(i32, usize)> {
    let mut low: usize = 0;
    let mut up: usize = arr.len() - 1;
    let mut i: usize = 0;

    while low <= up {
        i += 1;

        let mid: usize = (low + up) / 2;
        let mid_value = arr[mid];

        match mid_value.cmp(&disired_value) {
            Ordering::Equal => return Some((mid_value, mid)),
            Ordering::Greater => up = mid.checked_sub(1)?,
            Ordering::Less => low = mid + 1,
        }

        println!("Step {i}");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const ARR: [i32; 10] = [-1, 2, 3, 10, 50, 100, 130, 132, 150, 160];
    #[test]
    fn element_found() {
        assert_eq!((-1, 0), bin_search(&ARR, -1).unwrap());
    }

    #[test]
    fn element_not_found() {
        let result = bin_search(&ARR, 1000);
        assert!(result.is_none());
    }

    #[test]
    fn smallest_element_not_found() {
        let result = bin_search(&ARR, -1000);
        assert!(result.is_none());
    }
}
