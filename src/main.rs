mod infix_to_prefix;

use std::collections::VecDeque;
#[derive(Debug, Copy, Clone)]
enum Token {
  Op(Operator),
  Digit(i32),
}

#[derive(Debug, Copy, Clone)]
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

struct Lexer(VecDeque<char>);
impl Lexer {
  fn advance(&mut self) -> Result<char, &'static str> {
    if let Some(c) = self.0.pop_front() {
      Ok(c)
    } else {
      Err("end of source")
    }
  }
  fn peek(&self) -> Result<char, &'static str> {
    if self.0.len() > 1 {
      Ok(self.0[1])
    } else {
      Err("at end")
    }
  }
}

fn is_numeric(c: char) -> bool {
  "1234567890".contains(c)
}

fn tokenize(source: &str) -> Vec<Token> {
  let mut tokens: Vec<Token> = vec![];
  let source: VecDeque<char> = source.chars().collect(); // deque for Lexer pop_front()
  let mut lex: Lexer = Lexer(source);
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
  println!("{:?}", &tokens);
  tokens
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
    match tokens[0] {
      Token::Digit(d) => d,
      _ => 0,
    }
  }
}

fn calc(source: &str) -> i32 {
  let prefix_expr = infix_to_prefix::convert(source);
  let tokens = tokenize(&prefix_expr);
  Expr(tokens).solve()
}
fn main() {
  let res = calc("5 * 0");
  println!("\n{}", res);
}

#[test]
fn test0() {
  assert_eq!(calc("1 + 2 + 3"), 6);
}

#[test]
fn test1() {
  assert_eq!(calc("3 * 5 / 5 - 3"), 0);
}
