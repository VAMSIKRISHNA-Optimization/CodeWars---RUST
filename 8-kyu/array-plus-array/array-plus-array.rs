fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {
    xs.iter().sum::<i32>() + ys.iter().sum::<i32>()
}