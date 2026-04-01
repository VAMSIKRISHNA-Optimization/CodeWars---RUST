fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> 
{
    let mut ans = Vec::from(arr1);
    ans.extend(arr2);
    ans.sort();
    ans.dedup();
    ans
}