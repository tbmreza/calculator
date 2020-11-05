use std::collections::VecDeque;
use crate::enum_types::{Token, Operator};

enum Part {
    Operator,
    Number,
    Whitespace,
}

fn part(c: char) -> Part {
    if c == ' ' {
        Part::Whitespace
    } else if "+-*/".contains(c) {
        Part::Operator
    } else {
        Part::Number
    }
}

fn precedence(c: char) -> u32 {
    match c {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}


pub fn convert(infix_expression: &str) -> Result<String, Box<dyn std::error::Error>> {
    let input: VecDeque<char> = infix_expression.chars().collect();
    let mut _scanner = crate::scanner::Scanner(input);
    let mut stack: Vec<char> = vec![];
    let mut processed: Vec<char> = vec![];
    while _scanner.0.len() > 0 {
        let c = _scanner.advance_from_back().unwrap();
        match part(c) {
            Part::Whitespace => continue,
            Part::Number => {
                // processed.push(' ');
                processed.push(c);
            }
            Part::Operator => {
                processed.push(' ');
                if stack.len() == 0 {
                    stack.push(c);
                    continue;
                }
                let stack_top = stack.clone().pop().unwrap();
                if precedence(c) < precedence(stack_top) {
                    processed.push(stack.pop().unwrap());
                    processed.push(' ');
                    stack.push(c);
                } else {
                    stack.push(c);
                }
            }
        }
    }
    while stack.len() > 1 {
        processed.push(stack.pop().unwrap());
        processed.push(' ');
    }
    if stack.len() == 1 {
        processed.push(stack.pop().unwrap());
    }

    let prefix_expression: String = processed.into_iter().rev().collect();
    Ok(prefix_expression)
}
