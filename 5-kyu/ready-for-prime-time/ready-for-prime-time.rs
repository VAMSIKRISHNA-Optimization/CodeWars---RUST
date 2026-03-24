pub fn prime(n: u32) -> Vec<u32> 
{
    let mut Primes: Vec<u32> = Vec::new();
    for num in 2..=n
    {
        if is_prime(num) { Primes.push(num); }
    }
    Primes
}
​
    fn is_prime(num: u32)->bool
    {
        for div in (2..num).rev()
        {
            //println!("{}", div);
            if num % div == 0 { return false;}
        }
        return true;
    }