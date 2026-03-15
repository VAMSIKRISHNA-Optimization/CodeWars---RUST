fn sum_cubes(n: u32) -> u32 
{
    (1..=n).map(|v| v*v*v).sum()
}