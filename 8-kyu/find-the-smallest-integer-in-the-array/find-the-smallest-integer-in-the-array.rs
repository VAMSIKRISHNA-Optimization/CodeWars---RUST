fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().expect("Try again with a valid array")
}