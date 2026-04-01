fn quarter_of(month: u8) -> u8 {
    if month % 3 == 0 {
        month / 3
    } else {
        (month / 3) +  1
    }
}