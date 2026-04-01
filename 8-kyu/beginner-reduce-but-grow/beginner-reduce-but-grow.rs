fn grow(nums: Vec<i32>) -> i32 
{
    nums.iter().fold(1, |mut acc, e| acc * e)
}