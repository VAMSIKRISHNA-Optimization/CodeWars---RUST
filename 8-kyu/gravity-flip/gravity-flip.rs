fn flip(dir: char, cubes: &[u32]) -> Vec<u32> 
{
    /* My Solution (Time: O(n log n), Space:O(n)): 1774 ms */
    let mut vec = cubes.to_vec();
    match dir
    {
        'L' | 'l' => vec.sort_by(|a, b| b.cmp(a)),
        'R' | 'r' => vec.sort(),
        _ => unimplemented!(),
    }
    vec
    
    /* The Most Efficient Solution (Counting Sort) (Time: O(n+k), Space:O(n+k)): 1786 ms */
//     if cubes.is_empty() { return vec![]; }
    
//     let max = *cubes.iter().max().unwrap() as usize;
//     let mut counts = vec![0u32; max + 1];
//     for &num in cubes { counts[num as usize] += 1; }
​
//     let mut result = Vec::with_capacity(cubes.len());
//     let range: Box<dyn Iterator<Item = usize>> = match dir {
//         'R' | 'r' => Box::new(0..=max),
//         'L' | 'l' => Box::new((0..=max).rev()),
//         _ => unreachable!(),
//     };
​
//     for i in range {
//         for _ in 0..counts[i] {
//             result.push(i as u32);
//         }
//     }
//     result
}