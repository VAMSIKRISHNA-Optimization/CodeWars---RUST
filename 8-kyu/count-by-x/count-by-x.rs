fn count_by(x: u32, n: u32) -> Vec<u32> {
    (x..=x*n).step_by(x as usize).collect::<Vec<u32>>()
}