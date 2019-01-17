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
  current_token: Result<Option<Token>, &'a str>,
  current_char: char,
  num: Option<[TokenNumValue; 10]>,
  op: HashMap<String, TokenOpValue>,
  last_op: TokenOpValue,
  state: u8, // 0 = number, 1 = operator
}

impl<'a> Interpreter<'a> {
  pub fn new(text: &'a str) -> Self {
    Self {
      text: text,
      pos: 0,
      current_token: Ok(None),
      current_char: '0',
      num: Some([Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine]),
      op: hashmap![
        // "" => NoOp,
        String::from(" ") => NoOp,
        String::from("+") => Plus
      ],
      last_op: NoOp,
      state: 0,
    }
  }

  fn get_next_token(&mut self) -> Result<Option<Token>, &'a str> {
    let text = self.text;
    println!("text: {:?}", text);
    println!("pos: {}", self.pos);

    let char_count = text.chars().count();

    if char_count == 0 {
      println!("EOF 1");
      return Ok(Some(Token::new_op(Eof, NoOp)));
    }

    if self.pos > char_count - 1 {
      println!("EOF 2");

      self.state = (self.state + 1) % 2;
      let current_char = text.chars().nth(self.pos - 1).unwrap();

      if current_char.is_digit(10) {
        let val = current_char.to_digit(10).unwrap();
        return Ok(Some(Token::new_num(
          Integer,
          self.num.clone().unwrap()[val as usize].clone(),
        )));
      }
      // return Ok(Some(Token::new_op(Eof, NoOp)));
    }

    let current_char = text.chars().nth(self.pos).unwrap();
    println!("current_char: {:?}", current_char.to_string());

    if current_char.is_digit(10) {
      println!("is_digit");
      match current_char.to_digit(10) {
        Some(digit) => match self.num.clone() {
          Some(val) => {
            println!("to_digit Some(val): {:?}", val[digit as usize]);
            let token = Some(Token::new_num(Integer, val[digit as usize].clone()));
            println!("token: {:?}", token);
            self.pos += 1;
            self.state = (self.state + 1) % 2;
            return Ok(token);
          }
          None => {
            println!("to_digit None");
            // self.pos += 1;
          }
        },
        None => {
          return Err("Error: Failed converting character to number");
        }
      }
    }

    match Plus.val() {
      Some(val) => {
        if current_char.to_string() == val {
          println!("plus equals");
          let token = Some(Token::new_op(Add, self.op.get(&val).unwrap().clone()));
          self.last_op = Plus;
          self.pos += 1;
          // self.state = (self.state + 1) % 2;
          return Ok(token);
        } else {
          println!("plus not equals");
          self.pos += 1;
          return Ok(Some(Token::new_op(Space, NoOp)));
          // return Ok(Some(Token::new_op(Space, NoOp)));
        }
      }
      None => {
        println!("plus none");
        // self.pos += 1;
        return Ok(Some(Token::new_op(Space, NoOp)));
        // return Ok(Some(Token::new_op(Space, NoOp)));
      }
    }
  }

  fn eat(&mut self, token_type: TokenType) -> Result<Option<i16>, &'a str> {
    match self.current_token.clone() {
      Ok(token) => match token {
        Some(token) => {
          if *token.typ.val() == *token_type.val() {
            self.current_token = self.get_next_token();
            // self.state = (self.state + 1) % 2;

            println!("token types are the same: state: {}", self.state);
            println!("current_token: {:?}", self.current_token);
            Ok(None)
          } else {
            // self.pos -= 1;
            // if token.typ.val() != Eof.val() {
            self.current_token = self.get_next_token();
            // }
            // self.state = (self.state + 1) % 2;

            println!("token types are different: state: {}", self.state);
            println!("current_token: {:?}", self.current_token);
            Ok(None)
          }
        }

        None => {
          // self.current_token = self.get_next_token();
          // self.state = (self.state + 1) % 2;
          println!("token type is none: state: {}", self.state);
          Ok(None)
        }
      },

      Err(err) => Err(err),
    }
  }

  pub fn expr(&mut self) -> Result<i16, &'a str> {
    {
      self.current_token = self.get_next_token();
    }

    let left = match self.current_token.clone() {
      Ok(token) => match token {
        Some(token) => match token.clone().val {
          TokenValue::TokenNumValue(_) => Some(token),

          TokenValue::TokenOpValue(val) => match val {
            _ => Some(Token::new_num(Integer, Zero)),
          },
        },

        None => None,
      },

      Err(err) => {
        return Err(err);
      }
    };

    {
      loop {
        match left.clone() {
          Some(val) => {
            self.eat(val.typ)?;

            if self.state == 1 {
              break;
            }
          }
          None => {
            break;
            // self.state = (self.state + 1) % 2;
          }
        }
      }
    }

    let op = match self.current_token.clone() {
      Ok(token) => match token {
        Some(token) => match token.val {
          TokenValue::TokenOpValue(_) => Some(token),

          TokenValue::TokenNumValue(val) => match val {
            _ => Some(Token::new_op(Space, NoOp)),
          },
        },

        None => None,
      },

      Err(err) => {
        return Err(err);
      }
    };

    {
      loop {
        match op.clone() {
          Some(val) => {
            self.eat(val.typ)?;
            if self.state == 0 {
              break;
            }
          }
          None => {
            break;
            // self.state = (self.state + 1) % 2;
          }
        }
      }
    }

    let right = match self.current_token.clone() {
      Ok(token) => match token {
        Some(token) => match token.clone().val {
          TokenValue::TokenNumValue(_) => Some(token),

          TokenValue::TokenOpValue(val) => match val {
            _ => Some(Token::new_num(Integer, Zero)),
          },
        },

        None => None,
      },

      Err(err) => {
        return Err(err);
      }
    };

    {
      loop {
        match right.clone() {
          Some(val) => {
            self.eat(val.typ)?;

            if self.state == 1 {
              break;
            }
          }
          None => {
            break;
            // self.state = (self.state + 1) % 2;
          }
        }
      }
    }

    let left = match left.unwrap_or(Token::new_num(Integer, Zero)).val {
      TokenValue::TokenNumValue(val) => val.val(),
      _ => 0,
    };
    let right = match right.unwrap_or(Token::new_num(Integer, Zero)).val {
      TokenValue::TokenNumValue(val) => val.val(),
      _ => 0,
    };
    println!("left: {}", left);
    println!("right: {}", right);

    match op.unwrap_or(Token::new_op(Space, NoOp)).typ {
      _ => {
        if self.last_op.val().unwrap_or(Space.val()) == Plus.val().unwrap_or(Space.val()) {
          println!("last op equals");
          Ok((left + right) as i16)
        } else {
          println!("last op not equals");
          Ok((left + right) as i16)
        }
      }
    }
  }
}
