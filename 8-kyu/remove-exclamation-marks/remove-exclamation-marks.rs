fn remove_exclamation_marks(input: &str) -> String {
    input.chars().filter(|&c| c != '!').collect::<String>()
}