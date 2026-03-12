use either::Either;
​
fn sort_by_area(seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    let mut areas: Vec<f64>     = Vec::new();
    let mut indices: Vec<usize> = (0..seq.len()).collect();
    
    for ind in 0..seq.len()
    {
        match seq[ind]
        {
            Either::Left((l, b))  => areas.push(l*b),
            Either::Right(r)      => areas.push(3.14159265358979323846264338327950288_f64*r*r),
        }
    }
    
    indices.sort_by(|&x, &y| areas[x].partial_cmp(&areas[y]).unwrap());
    
    let mut answer: Vec<Either<(f64, f64), f64>> = Vec::new();
    
    for index in indices
    {
       answer.push(seq[index]);
    }
    
    return answer
}