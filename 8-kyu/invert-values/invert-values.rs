fn invert(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .map(|e| e * -1)
        .collect()
}