fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> 
{
    let mut ans = array
                    .iter()
                    .filter(|v| *v%2 == 0)
                    .rev()
                    .take(number)
                    .cloned()
                    .collect::<Vec<i32>>();
    ans.reverse();
    ans
}