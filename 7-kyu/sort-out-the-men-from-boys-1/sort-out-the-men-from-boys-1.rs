fn men_from_boys(xs: &[i16]) -> Vec<i16> 
{
    // 1. Separate into two vectors based on even/odd
    let (mut evens, mut odds): (Vec<i16>, Vec<i16>) = xs.iter()
        .cloned()
        .partition(|&n| n % 2 == 0);
​
    // 2. Sort evens ASCENDING (default)
    evens.sort_unstable();
    evens.dedup(); 
​
    // 3. Sort odds DESCENDING (using a custom comparator)
    odds.sort_unstable_by(|a, b| b.cmp(a));
    odds.dedup();
​
    // 4. Combine them
    evens.extend(odds);
    evens
}