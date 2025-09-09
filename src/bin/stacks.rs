use std::collections::{HashMap, HashSet};



fn main() {

    let test_valid: Vec<String> = vec!["()", "()[]{}", "{[]}", "([{}])", "()()", "{}([])", "[]{}()", "({[]})"].iter().map(|s| s.to_string()).collect();

    for s in &test_valid {
        println!("{}", isValid(&s));
        assert_eq!(isValid(&s), true, "{}", format!("{} is valid", *s));
    }

    let test_invalid: Vec<String> = vec![
        "([)]",
        "{(})",
        "(]",
        "{)",
        "[}",
        "[",
        "{()",
        ")(",
        "}{",
        "]",
        "(()",
        "({[)]",
        ].iter().map(|s| s.to_string()).collect();

    for s in &test_invalid {
        println!("{}", isValid(&s));
        assert_eq!(isValid(&s), false, "{}", format!("{} is not valid", *s));
    }
}



fn isValid(s: &String) -> bool {

    let open_chars: HashSet<char> = vec!['(', '[', '{', '<'].iter().copied().collect();
    let close_chars: HashSet<char> = vec![')', ']', '}', '>'].iter().copied().collect();

    let parenthesis_map: HashMap<char, char> = [
        (')', '('),
        (']', '['),
        ('}', '{'),
    ].iter().copied().collect();

    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if open_chars.contains(&c) {
            stack.push(c);
        } else if close_chars.contains(&c) {
            if stack.is_empty() {
                return false;
            }
            let top: char = stack.pop().unwrap();
            if top != *parenthesis_map.get(&c).unwrap(){
                return false;
            }
        }
    }
    stack.is_empty()
}