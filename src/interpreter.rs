use crate::token::{Token, TokenNumValue, TokenOpValue, TokenType, TokenValue};
use std::collections::HashMap;
use TokenNumValue::*;
use TokenOpValue::*;
use TokenType::*;

macro_rules! hashmap {
  ($( $key: expr => $val: expr ),*) => {{
    let mut map = HashMap::new();
    $( map.insert($key, $val); )*
    map
  }}
}

#[derive(Clone)]
pub struct Interpreter<'a> {
  text: &'a str,
  pos: usize,
  current_token: Result<Token, String>,
  current_char: char,
  num: Option<[TokenNumValue; 10]>,
  op: HashMap<&'a str, TokenOpValue>,
  last_op: TokenOpValue,
  state: u8, // 0 = number, 1 = operator
}

impl<'a> Interpreter<'a> {
  pub fn new(text: &'a str) -> Self {
    Self {
      text: text,
      pos: 0,
      current_token: Ok(Token::new_op(Eof, NoOp)),
      current_char: '0',
      num: Some([Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine]),
      op: hashmap![
        " " => NoOp,
        "+" => Plus
      ],
      last_op: NoOp,
      state: 0,
    }
  }

  fn get_next_token(&mut self) -> Result<Token, String> {
    let text = self.text;
    // println!("text: {:?}", text);
    // println!("pos: {}", self.pos);

    let char_count = text.chars().count();

    if char_count == 0 {
      // println!("EOF 1");
      return Ok(Token::new_op(Eof, NoOp));
    }

    if self.pos > char_count - 1 {
      // println!("EOF 2");
      self.state = (self.state + 1) % 2;
      return Ok(Token::new_op(Eof, NoOp));
    }

    let current_char = text.chars().nth(self.pos).unwrap();
    // println!("current_char: {:?}", current_char.to_string());

    if current_char.is_digit(10) {
      // println!("is_digit");
      match current_char.to_digit(10) {
        Some(digit) => match self.num.clone() {
          Some(val) => {
            // println!("to_digit Some(val): {:?}", val[digit as usize]);
            let token = Token::new_num(Integer, val[digit as usize].clone());
            // println!("token: {:?}", token);
            self.pos += 1;
            self.state = (self.state + 1) % 2;
            return Ok(token);
          }
          None => (),
        },
        None => {
          return Err(String::from("Error: Failed converting character to number"));
        }
      }
    }

    if current_char.to_string() == Plus.val() {
      // println!("plus equals");
      let token = Token::new_op(Add, self.op.get::<str>(&Plus.val()).unwrap().clone());
      self.last_op = Plus;
      self.pos += 1;
      return Ok(token);
    } else {
      // println!("plus not equals");
      self.pos += 1;
      return Ok(Token::new_op(Space, NoOp));
    }
  }

  fn eat(&mut self, token_type: TokenType) -> Result<(), String> {
    match self.current_token.clone() {
      Ok(token) => {
        if token.typ.val() == token_type.val() {
          self.current_token = self.get_next_token();
          // println!("token types are the same: state: {}", self.state);
          // println!("current_token: {:?}", self.current_token);
          Ok(())
        } else {
          self.current_token = self.get_next_token();
          // println!("token types are different: state: {}", self.state);
          // println!("current_token: {:?}", self.current_token);
          Ok(())
        }
      }
      Err(err) => Err(err),
    }
  }

  pub fn expr(&mut self) -> Result<i16, String> {
    self.current_token = self.get_next_token();

    let left = match self.current_token.clone() {
      Ok(token) => match token.val {
        TokenValue::TokenNumValue(_) => token,

        TokenValue::TokenOpValue(val) => match val {
          _ => Token::new_num(Integer, Zero),
        },
      },

      Err(err) => {
        return Err(err);
      }
    };

    loop {
      self.eat(left.typ.clone())?;

      if self.state == 1 {
        break;
      }
    }

    let op = match self.current_token.clone() {
      Ok(token) => match token.val {
        TokenValue::TokenOpValue(_) => token,

        TokenValue::TokenNumValue(val) => match val {
          _ => Token::new_op(Space, NoOp),
        },
      },

      Err(err) => {
        return Err(err);
      }
    };

    loop {
      self.eat(op.typ.clone())?;

      if self.state == 0 {
        break;
      }
    }

    let right = match self.current_token.clone() {
      Ok(token) => match token.val {
        TokenValue::TokenNumValue(_) => token,

        TokenValue::TokenOpValue(val) => match val {
          _ => Token::new_num(Integer, Zero),
        },
      },

      Err(err) => {
        return Err(err);
      }
    };

    loop {
      self.eat(right.typ.clone())?;

      if self.state == 1 {
        break;
      }
    }

    let left = match left.val {
      TokenValue::TokenNumValue(val) => val.val(),
      _ => 0,
    };

    let right = match right.val {
      TokenValue::TokenNumValue(val) => val.val(),
      _ => 0,
    };
    // println!("left: {}", left);
    // println!("right: {}", right);

    if self.last_op.val() == Plus.val() {
      // println!("last op equals");
      Ok((left + right) as i16)
    } else {
      // println!("last op not equals");
      Err(String::from("Unknown operator"))
    }
  }
}
