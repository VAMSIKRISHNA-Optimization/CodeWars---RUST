                else
                {
                    let mut i: i64 = 3;
                    while i * i <= quo
                    {
                        if quo % i == 0 { lofp.push(i); quo = quo/i; }
                        i += 2; 
                    }
                }
                
                if is_prime(quo) { lofp.push(quo); not_prime = false; }
            }
        }
      
    }
    
    lofp.sort();
    lofp.dedup();
    
    // Calculating the result
    let mut res: Vec<(i64, i64)> = Vec::new(); 
    for pri in 0..lofp.len()
    {
        let mut sum: i64 = 0;
        for ind in 0..nums.len()
        {
            if nums[ind] % lofp[pri] == 0 { sum = sum+nums[ind]; }
        }
        res.push((lofp[pri],sum));
    }
    
    return res;
}