fn bonus_time(salary: u64, bonus: bool) -> String {
    if bonus {
        format!("¥{}", salary*10)
    } else {
        format!("¥{}", salary)
    }
}