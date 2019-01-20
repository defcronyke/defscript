use std::cmp::PartialEq;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Token {
  pub typ: TokenType,
  pub val: TokenValue,
}

impl Token {
  pub fn new_num(typ: TokenType, val: TokenNumValue) -> Self {
    Token {
      typ,
      val: TokenValue::TokenNumValue(val),
    }
  }

  pub fn new_op(typ: TokenType, val: TokenOpValue) -> Self {
    Token {
      typ,
      val: TokenValue::TokenOpValue(val),
    }
  }

  pub fn string(&self) -> String {
    match self.val.clone() {
      TokenValue::TokenNumValue(val) => format!("Token({}, {})", self.typ.val(), val),

      TokenValue::TokenOpValue(val) => format!("Token({}, {})", self.typ.val(), val.val()),
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.string())
  }
}

#[derive(Clone, Debug)]
pub enum TokenType {
  Integer,
  Add,
  Subtract,
  Multiply,
  Divide,
  Eof,
}

impl TokenType {
  pub fn val<'a>(&self) -> &'a str {
    match *self {
      TokenType::Integer => "Integer",
      TokenType::Add => "Add",
      TokenType::Subtract => "Subtract",
      TokenType::Multiply => "Multiply",
      TokenType::Divide => "Divide",
      TokenType::Eof => "Eof",
    }
  }
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.val())
  }
}

impl PartialEq for TokenType {
  fn eq(&self, other: &TokenType) -> bool {
    self.val() == other.val()
  }
}

#[derive(Clone, Debug)]
pub enum TokenValue {
  TokenNumValue(TokenNumValue),
  TokenOpValue(TokenOpValue),
}

pub type TokenNumValue = i64;

#[derive(Debug, Clone)]
pub enum TokenOpValue {
  NoOp,
  Plus,
  Minus,
  Asterisk,
  Slash,
}

impl TokenOpValue {
  pub fn val<'a>(&self) -> &'a str {
    match *self {
      TokenOpValue::NoOp => "",
      TokenOpValue::Plus => "+",
      TokenOpValue::Minus => "-",
      TokenOpValue::Asterisk => "*",
      TokenOpValue::Slash => "/",
    }
  }
}

impl PartialEq for TokenOpValue {
  fn eq(&self, other: &TokenOpValue) -> bool {
    self.val() == other.val()
  }
}
