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
    match &self.val {
      TokenValue::TokenNumValue(val) => match val.val() {
        Some(val) => format!("Token({}, {})", self.typ.val(), val),
        None => format!("Token({}, None)", self.typ.val()),
      },

      TokenValue::TokenOpValue(val) => match val.val() {
        Some(val) => format!("Token({}, {})", self.typ.val(), val),
        None => format!("Token({}, None)", self.typ.val()),
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
  Integer,
  Add,
  Eof,
}

impl TokenType {
  pub fn val(&self) -> String {
    match *self {
      TokenType::Integer => String::from("Integer"),
      TokenType::Add => String::from("Add"),
      TokenType::Eof => String::from("Eof"),
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
  NoNum,
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
  pub fn val(&self) -> Option<u8> {
    match *self {
      TokenNumValue::NoNum => None,
      TokenNumValue::Zero => Some(0),
      TokenNumValue::One => Some(1),
      TokenNumValue::Two => Some(2),
      TokenNumValue::Three => Some(3),
      TokenNumValue::Four => Some(4),
      TokenNumValue::Five => Some(5),
      TokenNumValue::Six => Some(6),
      TokenNumValue::Seven => Some(7),
      TokenNumValue::Eight => Some(8),
      TokenNumValue::Nine => Some(9),
    }
  }
}

#[derive(Debug, Clone)]
pub enum TokenOpValue {
  NoOp,
  Plus,
}

impl TokenOpValue {
  pub fn val(&self) -> Option<&str> {
    match *self {
      TokenOpValue::NoOp => None,
      TokenOpValue::Plus => Some("+"),
    }
  }
}
