mod infix_to_prefix;
mod scanner;

use std::collections::VecDeque;
#[derive(Debug, Copy, Clone, std::cmp::PartialEq)]
enum Token {
    Op(Operator),
    Digit(i32),
}

#[derive(Debug, Copy, Clone, std::cmp::PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exp,
}
fn is_op_token(t: Token) -> bool {
    match t {
        Token::Op(_) => true,
        _ => false,
    }
}
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
    "1234567890".contains(c)
}

fn tokenize(source: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut tokens: Vec<Token> = vec![];
    let source: VecDeque<char> = source.chars().collect(); // deque for Scanner pop_front()
    let mut lex: scanner::Scanner = scanner::Scanner(source);
    let mut num_literal = "".to_string();
    while lex.0.len() > 0 {
        let p = lex.peek();
        let c = lex.advance().unwrap();
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
    println!("tokenize: {:?}", &tokens);
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
                if is_op_token(*x) {
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
    // let prefix_expr = "".to_string();
    let tokens = tokenize(&prefix_expr)?;
    Ok(Expr(tokens).solve())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = calc("5 * 0")?;
    println!("\n{}", res);
    Ok(())
}

#[test]
fn test0() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(calc("1 + 2 + 3")?, 6);
    assert_eq!(calc("3 * 5 / 5 - 3")?, 0);
    // assert_eq!(calc("123 * 10")?, 1230);
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
fn test_single() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(calc("0")?, 0);
    assert_eq!(calc("9")?, 9);
    // assert_eq!(calc("12")?, 12);
    // assert_eq!(calc("9090")?, 9090);
    Ok(())
}
#[test]
fn test_empty() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(calc("")?, 0);
    Ok(())
}

// TODO: Scanner module to be imported to main and infix_to_prefix
// TODO: refactor infix_to_prefix::convert() to be like tokenize()
