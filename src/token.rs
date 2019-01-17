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
      TokenValue::TokenNumValue(val) => format!("Token({}, {})", self.typ.val(), val.val()),

      TokenValue::TokenOpValue(val) => match val.val() {
        _ => format!("Token({}, {})", self.typ.val(), val.val()),
      },
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
  Space,
  Integer,
  Add,
  Eof,
}

impl TokenType {
  pub fn val<'a>(&self) -> &'a str {
    match *self {
      TokenType::Space => " ",
      TokenType::Integer => "Integer",
      TokenType::Add => "Add",
      TokenType::Eof => "Eof",
    }
  }
}

#[derive(Clone, Debug)]
pub enum TokenValue {
  TokenNumValue(TokenNumValue),
  TokenOpValue(TokenOpValue),
}

#[derive(Debug, Clone)]
pub enum TokenNumValue {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
}

impl TokenNumValue {
  pub fn val(&self) -> u8 {
    match *self {
      TokenNumValue::Zero => 0,
      TokenNumValue::One => 1,
      TokenNumValue::Two => 2,
      TokenNumValue::Three => 3,
      TokenNumValue::Four => 4,
      TokenNumValue::Five => 5,
      TokenNumValue::Six => 6,
      TokenNumValue::Seven => 7,
      TokenNumValue::Eight => 8,
      TokenNumValue::Nine => 9,
    }
  }
}

#[derive(Debug, Clone)]
pub enum TokenOpValue {
  NoOp,
  Plus,
}

impl TokenOpValue {
  pub fn val<'a>(&self) -> &'a str {
    match *self {
      TokenOpValue::NoOp => " ",
      TokenOpValue::Plus => "+",
    }
  }
}
