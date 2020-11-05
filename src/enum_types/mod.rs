#[derive(Debug, Copy, Clone, std::cmp::PartialEq)]
pub enum Token {
    Op(Operator),
    Digit(i32),
}

#[derive(Debug, Copy, Clone, std::cmp::PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exp,
}