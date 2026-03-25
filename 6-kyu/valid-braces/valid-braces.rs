fn valid_braces(s: &str) -> bool 
{
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            _ => continue, // Ignore non-bracket characters
        }
    }
    stack.is_empty()
}