    }
   
   
  s.parse::<u32>().unwrap()
}
​
fn count_digits(mut n: u32) -> u32
{
    if n == 0 { return 1; }
    let mut count = 0;
    while n > 0
    {
        n /= 10;
        count += 1;
    }
    count
}
​
fn all_nines(mut n: u32) -> bool
{
    if n == 0 { return false; } // 0 is not 9
    while n > 0
    {
        if n % 10 != 9
        {
            return false;
        }
        n /= 10;
    }
    true
}
​