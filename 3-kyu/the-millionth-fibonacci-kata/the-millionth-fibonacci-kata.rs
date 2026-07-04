        fib_abs
    }
}
​
// Computes both ( F(n), F(n+1) ) efficiently in O(log n)
fn fast_fib(n: u32) -> (BigInt, BigInt) 
{
    if n == 0 
    {
        return (BigInt::zero(), BigInt::one());
    }
​
    // Solve for k = n / 2
    let (fk, fk_plus_1) = fast_fib(n / 2);
​
    // Pre-calculate reused doubling parts
    let two = BigInt::from(2);
    // F(2k) = F(k) * [2 * F(k+1) - F(k)]
    let f_2k = &fk * ((&two * &fk_plus_1) - &fk);
    // F(2k+1) = F(k)^2 + F(k+1)^2
    let f_2k_plus_1 = (&fk * &fk) + (&fk_plus_1 * &fk_plus_1);
​
    if n % 2 == 0 
    {
        // If n is even, return ( F(2k), F(2k+1) )
        (f_2k, f_2k_plus_1)
    } 
    else 
    {
        // If n is odd, the next sequential value after F(2k+1) is F(2k) + F(2k+1)
        let next = &f_2k + &f_2k_plus_1;
        (f_2k_plus_1, next)
    }
}
​