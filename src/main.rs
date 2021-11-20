use std::env;
extern crate nom;

use nom::branch::alt;
use nom::{
  bytes::complete::{tag, take_while_m_n},
  character::complete::{alpha1, char},
  combinator::map_res,
  sequence::pair,
  IResult,
};

mod lexeme;

#[derive(Debug, PartialEq)]
pub enum Expr {
  MyInt(u8),                     // 123
  MyVar(String),                 // 'A' or 'B' etc
  MyFunction(String, Box<Self>), // '\A -> A'
}

fn is_int_digit(c: char) -> bool {
  c.is_digit(10)
}

fn int_primary(input: &str) -> IResult<&str, u8> {
  map_res(take_while_m_n(1, 12, is_int_digit), from_int)(input)
}

fn from_int(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 10)
}

fn parse_my_int(input: &str) -> IResult<&str, Expr> {
  let (input, int_val) = lexeme::ws(int_primary)(input)?;
  Ok((input, Expr::MyInt(int_val)))
}

#[test]
fn test_parse_my_int() {
  assert_eq!(parse_my_int("1"), Ok(("", Expr::MyInt(1))));
  assert_eq!(parse_my_int("11"), Ok(("", Expr::MyInt(11))));
  assert_eq!(parse_my_int("11dog"), Ok(("dog", Expr::MyInt(11))));
}

fn parse_my_var(input: &str) -> IResult<&str, Expr> {
  let (input, var_val) = lexeme::ws(alpha1)(input)?;
  Ok((input, Expr::MyVar(var_val.to_string())))
}

#[test]
fn test_parse_my_var() {
  assert_eq!(parse_my_var("p"), Ok(("", Expr::MyVar("p".to_string()))));
  assert_eq!(
    parse_my_var("poo"),
    Ok(("", Expr::MyVar("poo".to_string())))
  );
  assert_eq!(
    parse_my_var("poo "),
    Ok((" ", Expr::MyVar("poo".to_string())))
  )
}

fn parse_my_fn(input: &str) -> IResult<&str, Expr> {
  let (input, (_slash, arg_val)) = pair(lexeme::ws(char('\\')), alpha1)(input)?;
  let (input, _) = lexeme::ws(tag("->"))(input)?;
  let (input, expr) = parse_my_expr(input)?;
  Ok((input, Expr::MyFunction(arg_val.to_string(), Box::new(expr))))
}

#[test]
fn test_parse_my_fn() {
  assert_eq!(
    parse_my_fn("\\a -> 1"),
    Ok((
      "",
      Expr::MyFunction("a".to_string(), Box::new(Expr::MyInt(1)))
    ))
  );
  assert_eq!(
    parse_my_fn("\\a -> \\b -> a"),
    Ok((
      "",
      Expr::MyFunction(
        "a".to_string(),
        Box::new(Expr::MyFunction(
          "b".to_string(),
          Box::new(Expr::MyVar("a".to_string()))
        ))
      )
    ))
  );
}

fn parse_my_expr(input: &str) -> IResult<&str, Expr> {
  alt((parse_my_int, parse_my_var, parse_my_fn))(input)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  let mut parse_expr = alt((alt((parse_my_int, parse_my_var)), parse_my_fn));

  let first_arg = &args[1];
  let result = parse_expr(first_arg);
  println!("{:?}", result);
}
