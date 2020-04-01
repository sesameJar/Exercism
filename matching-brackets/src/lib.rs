use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut braces = HashMap::new();
    braces.insert('{', '}');
    braces.insert('[', ']');
    braces.insert('(', ')');
    let mut pairs: Vec<char> = vec![];

    for c in string.chars() {
        if let Some(_bracket) = braces.get(&c) {
            pairs.push(c)
        }

        if braces.values().any(|&x| x == c) {
            if !pairs.is_empty() && braces.get(pairs.last().unwrap()) == Some(&c) {
                pairs.pop();
            } else {
                return false;
            }
        }
    }

    if !pairs.is_empty() {
        return false;
    }

    true
}
