                                        message
                                    }
                                })
                                .collect::<String>();
                                
                         
    ans
}
​
fn count_carries(mut a: u64, mut b: u64) -> u8 
{
    let mut carries = 0;
    let mut current_carry = 0;
​
    // Process digit by digit from right to left
    while a > 0 || b > 0 {
        let digit_a = a % 10;
        let digit_b = b % 10;
        
        let sum = digit_a + digit_b + current_carry;
        
        if sum >= 10 {
            current_carry = 1;
            carries += 1;
        } else {
            current_carry = 0;
        }
        
        a /= 10;
        b /= 10;
    }
    carries
}