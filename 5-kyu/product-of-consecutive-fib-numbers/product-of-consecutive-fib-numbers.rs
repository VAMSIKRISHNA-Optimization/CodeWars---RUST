fn product_fib(prod: u64) -> (u64, u64, bool) 
{
    let mut fib1 = 0;
    let mut fib2 = 1;
​
    while fib1 * fib2 < prod 
    {
        let next_fib = fib1 + fib2;
        fib1 = fib2;
        fib2 = next_fib;
    }
​
    return (fib1, fib2, fib1 * fib2 == prod);
}