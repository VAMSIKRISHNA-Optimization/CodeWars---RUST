             }
        5 => return 5 as i32,
        6 => return 6 as i32,
        7 => {              
                let digits_7 = [7,9,3,1];
                let pow_last_digit = str2.chars().rev().take(2).collect::<String>()
                                     .chars().rev().collect::<String>()
                                     .parse().unwrap_or(0);
                let mut ind: usize = 0;
                if pow_last_digit % 4 == 0 { ind = 4; } else { ind = pow_last_digit%4;}
                return digits_7[ind- 1];
             }
        8 => {              
                let digits_8 = [8,4,2,6];
                let pow_last_digit = str2.chars().rev().take(2).collect::<String>()
                                     .chars().rev().collect::<String>()
                                     .parse().unwrap_or(0);
                let mut ind: usize = 0;
                if pow_last_digit % 4 == 0 { ind = 4; } else { ind = pow_last_digit%4;}
                return digits_8[ind- 1]; 
             }
        9 => {              
                let digits_9 = [9,1];
                let pow_last_digit = str2.chars().rev().take(2).collect::<String>()
                                     .chars().rev().collect::<String>()
                                     .parse().unwrap_or(0);
                let mut ind: usize = 0;
                if pow_last_digit % 2 == 0 { ind = 2; } else { ind = pow_last_digit%2;}
                return digits_9[ind- 1];
             }
                
        _ => return -1 as i32,
    }
}