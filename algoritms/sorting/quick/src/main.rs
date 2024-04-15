#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let array = vec![10, 5, 2, 3, 2];
    let result = quicksort(array);
    println!("Sorted array: {result:?}");
}
fn quicksort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr.pop().unwrap();
    let (less, greater): (Vec<i32>, Vec<i32>) = arr.into_iter().partition(|&x| x <= pivot);

    let mut sorted_arr = quicksort(less);
    sorted_arr.push(pivot);
    sorted_arr.extend(quicksort(greater));

    sorted_arr
}
