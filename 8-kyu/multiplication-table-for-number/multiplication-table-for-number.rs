fn multi_table(n: u64) -> String {
    let mul: Vec<u64> = (1..=10).collect();
    let sentence = mul.iter().fold(String::new(), |mut acc, m| 
                   {
                        let message = format!("{} * {} = {}{}", *m, n, *m*n, if *m == 10 {""} else {"\n"});
                        acc.push_str(&message);
                        acc
                   });
    return sentence;
}