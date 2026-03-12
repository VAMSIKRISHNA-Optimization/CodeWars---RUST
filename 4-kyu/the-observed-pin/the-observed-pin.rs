​
        if loop_count >= max_combos as i32
        {
            break 'outer;
        }
​
​
    }
​
    return ans;
}
​
​
fn get_alternative_nums(cur_num: u8)->Vec<u8>
{
    let mut alternative_nums: Vec<u8> = Vec::new();
    match cur_num
    {
        1 => alternative_nums.extend(&[1,2,4]),
        2 => alternative_nums.extend(&[1,2,3,5]),
        3 => alternative_nums.extend(&[2,3,6]),
        4 => alternative_nums.extend(&[1,4,5,7]),
        5 => alternative_nums.extend(&[2,4,5,6,8]),
        6 => alternative_nums.extend(&[3,5,6,9]),
        7 => alternative_nums.extend(&[4,7,8]),
        8 => alternative_nums.extend(&[0,5,7,8,9]),
        9 => alternative_nums.extend(&[6,8,9]),
        0 => alternative_nums.extend(&[0,8]),
        _ => alternative_nums.push(111),
    }
    alternative_nums
}
​