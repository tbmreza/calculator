use std::collections::VecDeque;

pub struct Scanner(pub VecDeque<char>);
impl Scanner {
    pub fn advance(&mut self) -> Result<char, &'static str> {
        if let Some(c) = self.0.pop_front() {
            Ok(c)
        } else {
            Err("end of source")
        }
    }
    pub fn peek(&self) -> Result<char, &'static str> {
        if self.0.len() > 1 {
            Ok(self.0[1])
        } else {
            Err("at end")
        }
    }
}