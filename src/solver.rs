use crate::enum_types::{Operator, Token};
use crate::scanner;
use std::collections::VecDeque;

fn operate(op: Token, a: Token, b: Token) -> Token {
    if let (Token::Op(op), Token::Digit(a), Token::Digit(b)) = (op, a, b) {
        match op {
            Operator::Add => Token::Digit(a + b),
            Operator::Subtract => Token::Digit(a - b),
            Operator::Multiply => Token::Digit(a * b),
            Operator::Divide => Token::Digit(a / b),
            Operator::Exp => Token::Digit(a.pow(b as u32)),
        }
    } else {
        Token::Digit(0)
    }
}

fn is_numeric(c: char) -> bool {
    c.is_ascii() && c.is_numeric()
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut tokens: Vec<Token> = vec![];
    let source: VecDeque<char> = source.chars().collect(); // deque for Scanner pop_front()
    let mut _scanner = scanner::Scanner(source);
    let mut num_literal = "".to_string();
    while _scanner.0.len() > 0 {
        let p = _scanner.peek();
        let c = _scanner.advance_from_front().unwrap();
        match c {
            ' ' => {}
            '+' => tokens.push(Token::Op(Operator::Add)),
            '-' => tokens.push(Token::Op(Operator::Subtract)),
            '*' => tokens.push(Token::Op(Operator::Multiply)),
            '/' => tokens.push(Token::Op(Operator::Divide)),
            '^' => tokens.push(Token::Op(Operator::Exp)),
            _ => {
                // number
                num_literal.push(c);
                if let (p, Ok(parsed)) = (p, num_literal.parse::<i32>()) {
                    if p.is_err() || !is_numeric(p.unwrap()) {
                        tokens.push(Token::Digit(parsed));
                        num_literal = "".to_string();
                    }
                }
            }
        }
    }
    // println!("tokenize: {:?}", &tokens);
    Ok(tokens)
}

#[derive(Debug)]
struct Expr(Vec<Token>);

impl Expr {
    fn solve(&self) -> i32 {
        let mut tokens = self.0.clone();
        while tokens.len() > 1 {
            let mut last_op_index: usize = 0;
            for (i, x) in tokens.iter().enumerate() {
                if let Token::Op(..) = x {
                    last_op_index = i;
                }
            }
            let op = tokens.remove(last_op_index);
            let operand1 = tokens.remove(last_op_index);
            let operand2 = tokens.remove(last_op_index);

            tokens.insert(last_op_index, operate(op, operand1, operand2));
        }
        let mut answer = 0;

        if tokens.len() == 1 {
            if let Token::Digit(d) = tokens[0] {
                answer = d;
            }
        }
        answer
    }
}

pub fn calc(source: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let prefix_expr = crate::infix_to_prefix::convert(source)?;
    let tokens = tokenize(&prefix_expr)?;
    Ok(Expr(tokens).solve())
}
