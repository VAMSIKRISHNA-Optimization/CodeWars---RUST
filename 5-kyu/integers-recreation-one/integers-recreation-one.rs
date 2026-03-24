fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> 
{
    let mut num = m;
    let mut ans: Vec<(u64, u64)> = Vec::new();
    
    //println!("{:?}", (1..=((n as f64).sqrt() as u64)));
    
    while num <= n
    {
        let sum_of_divisors = (1..=((num as f64).sqrt() as u64))
                              .filter(|&e| num % e == 0)
                              .fold(0, |acc, i| 
                              {
                                let d1_squared = i * i;
                                if d1_squared == num 
                                {
                                    acc + d1_squared
                                } else {
                                    let d2 = num / i;
                                    acc + d1_squared + (d2 * d2)
                                }
                              });
                              
            //println!("{:?}, {:?}", num, sum_of_divisors);
            let root = (sum_of_divisors as f64).sqrt() as u64;
            if root * root == sum_of_divisors
            {
              ans.push((num, sum_of_divisors));
            }
            
        num += 1;
    }
    
   ans 
}