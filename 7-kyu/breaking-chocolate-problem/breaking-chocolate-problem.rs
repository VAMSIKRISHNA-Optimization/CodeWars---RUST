fn break_chocolate(n: u32, m: u32) -> u64 {
    if n == 0 || m == 0 { return 0 };
    n as u64 * m as u64 - 1
}