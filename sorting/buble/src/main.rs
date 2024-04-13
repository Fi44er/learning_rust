#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let vector: Vec<i32> = vec![84, 42, 38, 77, 95];

    let result = buble_sort(vector);
    println!("Sorted array {result:?}");
}

fn buble_sort(mut vector: Vec<i32>) -> Vec<i32> {
    loop {
        let mut checker = true;
        for n in 1..vector.len() {
            if vector[n - 1] > vector[n] {
                vector.swap(n - 1, n);

                checker = false;
            }
        }
        if checker {
            break;
        }
    }
    vector
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_vector() {
        let unsorted_vec: Vec<i32> = vec![5, 2, 9, 1, 7, 3, 10, 4, 8, 6];
        let sorted_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sorted_vec, buble_sort(unsorted_vec));
    }
}
