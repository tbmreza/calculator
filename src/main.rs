mod enum_types;
mod infix_to_prefix;
mod scanner;

use enum_types::{Operator, Token};
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

fn tokenize(source: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
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

fn calc(source: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let prefix_expr = infix_to_prefix::convert(source)?;
    let tokens = tokenize(&prefix_expr)?;
    Ok(Expr(tokens).solve())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = calc("5 * 0")?;
    println!("\n{}", res);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! calc_against_builtin {
        ($expression:expr) => {
            // expands to assert_eq!(calc("123 * 10")?, 123 * 10);
            assert_eq!(calc(stringify!($expression))?, $expression);
            println!("calc({:?}) = {:?}", stringify!($expression), $expression);
        };
    }
    #[test]
    fn test_example() -> Result<(), Box<dyn std::error::Error>> {
        calc_against_builtin!(1 + 2 + 3);
        calc_against_builtin!(3 * 5 / 5 - 3);
        calc_against_builtin!(123 * 10);
        calc_against_builtin!(123 * 10 + 12 + 1111 + 909090 + 10 * 10);
        Ok(())
    }
    #[test]
    fn test_single() -> Result<(), Box<dyn std::error::Error>> {
        calc_against_builtin!(0);
        calc_against_builtin!(9);
        calc_against_builtin!(12);
        calc_against_builtin!(9090);
        Ok(())
    }
    #[test]
    fn test_non_int() -> Result<(), Box<dyn std::error::Error>> {
        calc_against_builtin!(12 / 5);
        calc_against_builtin!(12 / 5 * 10 / 3);
        calc_against_builtin!(12 / 5 * 10 / 3 + 1 - 2);
        calc_against_builtin!(9500 / 13 / 41);
        Ok(())
    }
    #[test]
    fn test_empty() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(calc("")?, 0);
        Ok(())
    }
    #[test]
    fn test_tokenize() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(tokenize("5")?, vec![Token::Digit(5)]);
        assert_eq!(tokenize("25")?, vec![Token::Digit(25)]);
        assert_eq!(tokenize("123")?, vec![Token::Digit(123)]);
        Ok(())
    }

    #[test]
    fn test_convert() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            infix_to_prefix::convert("12 + 34 + 56")?,
            "+ +12 34 56".to_string()
        );
        Ok(())
    }
}
