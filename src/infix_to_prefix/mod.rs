enum Part {
  Operator,
  Operand,
  Whitespace,
}

fn part(c: char) -> Part {
  if c == ' ' {
    Part::Whitespace
  } else if "+-*/".contains(c) {
    Part::Operator
  } else {
    Part::Operand
  }
}

fn power(c: char) -> u32 {
  match c {
    '+' | '-' => 1,
    '*' | '/' => 2,
    _ => 0,
  }
}
struct Lexer(Vec<char>);
impl Lexer {
  fn advance(&mut self) -> Result<char, &'static str> {
    if let Some(c) = self.0.pop() {
      Ok(c)
    } else {
      Err("end of source")
    }
  }
}
pub fn convert(infix_expression: &str) -> String {
  let input: Vec<char> = infix_expression.chars().collect();
  let mut lexer = Lexer(input);
  let mut stack: Vec<char> = vec![];
  let mut processed: Vec<char> = vec![];
  while lexer.0.len() > 0 {
    let c = lexer.advance().unwrap();
    match part(c) {
      Part::Whitespace => continue,
      Part::Operand => {
        processed.push(c);
        processed.push(' ');
      }
      Part::Operator => {
        if stack.len() == 0 {
          stack.push(c);
          continue;
        }
        let stack_top = stack.clone().pop().unwrap();
        if power(c) < power(stack_top) {
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
  processed.push(stack.pop().unwrap());

  let prefix_expression: String = processed.into_iter().rev().collect();
  prefix_expression
}
