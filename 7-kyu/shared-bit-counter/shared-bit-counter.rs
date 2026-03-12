​
    let mut BCD     = Vec::new();
    let mut str_BCD = String::new();
    
    while quotient != 1
    {
        quotient  = dividend / 2;
        remainder = dividend % 2; 
        dividend  = quotient;
                
        BCD.push(remainder);
    }
            
    BCD.push(1);
    //BCD.reverse();
​
    // println!("{:?}", BCD);
​
    let mut BCD_full32 = vec![0;mem::size_of_val(&dec)*8];
    let max_len        = BCD_full32.len()-1;
    for (index, value) in BCD.iter().enumerate()
    {
        // println!("{}", value);
        BCD_full32[max_len-index] += value;
    }
​
    for i in BCD_full32.iter()
    {
        str_BCD.push_str(&i.to_string());
    }
​
    return str_BCD;
}