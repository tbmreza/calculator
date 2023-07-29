mod enum_types;
mod infix_to_prefix;
mod scanner;
mod solver;

// use solver::{calc, tokenize};
use solver::{calc};
use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(version)]
struct Cli {
    input: Option<String>,
    #[arg(short, long)]
    load: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::{Parser};
    match Cli::parse() {
        Cli { input: Some(p), .. } => println!("{}", calc(&p)?),
        Cli { load: Some(p), .. } => println!("{}", p.display()),
        _ => unimplemented!()
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use enum_types::Token;
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
            infix_to_prefix::convert("12 + 34 + 56 + 0")?,
            "+ + +12 34 56 0".to_string()
        );
        Ok(())
    }
}
