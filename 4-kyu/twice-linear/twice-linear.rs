fn dbl_linear(n: u32) -> u32 {
    let mut dbl_vec:Vec<u32> = [1].to_vec();
​
    let mut ind2 = 0;
    let mut ind3 = 0;
​
    while dbl_vec.len() <= n as usize 
    {
        let next_2 = 2 * dbl_vec[ind2] + 1;
        let next_3 = 3 * dbl_vec[ind3] + 1;
​
        if next_2 < next_3 
        {
            dbl_vec.push(next_2);
            ind2 += 1;
        } 
        else if next_2 > next_3 
        {
            dbl_vec.push(next_3);
            ind3 += 1;
        } 
        else 
        {
            dbl_vec.push(next_2);
            ind2 += 1;
            ind3 += 1;
        }
    }
​
    dbl_vec[n as usize]
}