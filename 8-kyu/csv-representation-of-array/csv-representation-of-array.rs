// use std::fmt::Write;
​
fn to_csv_text(array: &[Vec<i8>]) -> String 
{
    /* My Solution (Time Complexity: O(N*M), Space Complexity: O(N*M)) : 1788 ms */
    array
    .iter()
    .map(|sv| 
    {
        sv
        .iter()
        .enumerate()
        .map(|(i,v)| 
        {
            if i == 0 { v.to_string()}
            else if i == sv.len()-1 { v.to_string() }
            else { v.to_string() }
        })
        .collect::<Vec<String>>()
        .join(",")
    })
    .collect::<Vec<String>>()
    .join("\n")
    
    /* The Most Effective Solution (Time Complexity: O(N*M), Space Complexity: O(Output Size)) : 1707 ms */
        // Pre-calculate an estimated capacity to minimize re-allocations
//     let mut csv = String::with_capacity(array.len() * 10); 
​
//     for (i, row) in array.iter().enumerate() {
//         for (j, &val) in row.iter().enumerate() {
//             let _ = write!(csv, "{}", val); // Writes directly into the 'csv' string
//             if j < row.len() - 1 {
//                 csv.push(',');
//             }
//         }
//         if i < array.len() - 1 {
//             csv.push('\n');
//         }
//     }
//     csv
    
}