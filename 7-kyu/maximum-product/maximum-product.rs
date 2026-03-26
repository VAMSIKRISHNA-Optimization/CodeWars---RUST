fn adjacent_elements_product(xs: &[i32]) -> i32 {
    xs.windows(2).map(|w| w[0] * w[1]).max().unwrap()
}