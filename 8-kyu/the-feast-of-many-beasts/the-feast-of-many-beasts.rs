fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().next() == dish.chars().next()
        && beast.chars().next_back() == dish.chars().next_back()
}