use std::collections::{HashMap, HashSet};



fn main() {

    let test_valid: Vec<String> = vec!["()", "()[]{}", "{[]}", "([{}])", "()()", "{}([])", "[]{}()", "({[]})"].iter().map(|s| s.to_string()).collect();

    for s in &test_valid {
        // println!("{}", is_valid(&s));
        assert_eq!(is_valid(&s), true, "{}", format!("{} is valid", *s));
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
        // println!("{}", is_valid(&s));
        assert_eq!(is_valid(&s), false, "{}", format!("{} is not valid", *s));
    }
}



fn is_valid(s: &String) -> bool {

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

struct MinStack {
    stack: Vec<i32>,
    internal_min_stack: Vec<i32>
}

impl MinStack {
    fn new() -> MinStack {
        MinStack {
            stack: Vec::new(),
            internal_min_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if (x < self.get_min()) || self.stack.is_empty() {
            self.internal_min_stack.push(x);
        } else {
            self.internal_min_stack.push(self.get_min());
        }
        self.stack.push(x);
    }

    fn pop(&mut self) -> Option<i32> {
        self.internal_min_stack.pop();
        self.stack.pop()
    }

    fn top(&self) -> i32 {
        self.stack[self.internal_min_stack.len() - 1]
    }

    fn get_min(&self) -> i32 {
        if self.internal_min_stack.is_empty() {
            return i32::MAX;
        }
        self.internal_min_stack[self.internal_min_stack.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();

        // 1. Initial state check
        assert_eq!(min_stack.pop(), None);
        assert_eq!(min_stack.get_min(), i32::MAX);
        // Assuming top() on an empty stack is an error or returns a default,
        // you might need to adjust this depending on your implementation.
        // For this test, we assume an initial `get_min` returns a high value.
        // And that `top()` is not called on an empty stack.

        // 2. Push some elements and verify
        min_stack.push(3);
        assert_eq!(min_stack.top(), 3);
        assert_eq!(min_stack.get_min(), 3);

        min_stack.push(5);
        assert_eq!(min_stack.top(), 5);
        assert_eq!(min_stack.get_min(), 3); // The minimum should still be 3

        min_stack.push(2); // New minimum
        assert_eq!(min_stack.top(), 2);
        assert_eq!(min_stack.get_min(), 2);

        // 3. Pop elements and verify
        let popped = min_stack.pop(); // Pop 2
        assert_eq!(popped, Some(2));
        assert_eq!(min_stack.top(), 5);
        println!("{:?}", min_stack.internal_min_stack);
        assert_eq!(min_stack.get_min(), 3); // Minimum should revert to 3

        let popped = min_stack.pop(); // Pop 5
        assert_eq!(popped, Some(5));
        assert_eq!(min_stack.top(), 3);
        assert_eq!(min_stack.get_min(), 3);

        // 4. Test edge case: popping the last element
        let popped = min_stack.pop(); // Pop 3
        assert_eq!(popped, Some(3));
        assert_eq!(min_stack.pop(), None);
        // The stack should be empty, and the minimum should reflect that
        assert_eq!(min_stack.get_min(), i32::MAX);
    }
}
