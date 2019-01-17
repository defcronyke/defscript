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
  num: HashMap<u8, TokenNumValue>,
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
      num: hashmap![
        0 => Zero,
        1 => One,
        2 => Two,
        3 => Three,
        4 => Four,
        5 => Five,
        6 => Six,
        7 => Seven,
        8 => Eight,
        9 => Nine
      ],
      op: hashmap![
        String::from("+") => Plus,
        String::from(" ") => NoOp
      ],
      last_op: NoOp,
      state: 0,
    }
  }

  fn get_next_token(&mut self) -> Result<Option<Token>, &'a str> {
    let text = self.text;
    // println!("text: {:?}", text);

    if self.pos > text.chars().count() - 1 {
      // println!("EOF");
      return Ok(Some(Token::new_op(Eof, NoOp)));
    }

    let current_char = text.chars().nth(self.pos).unwrap();

    if current_char.is_digit(10) {
      // println!("is_digit");
      match current_char.to_digit(10) {
        Some(val) => {
          let val: Option<&TokenNumValue> = self.num.get(&(val as u8));
          // println!("to_digit Some(val): {:?}", val);
          match val {
            Some(val) => {
              // println!("num get val: {:?}", val);
              let token = Some(Token::new_num(Integer, val.clone()));
              // println!("token: {:?}", token);
              self.pos += 1;
              return Ok(token);
            }
            None => {
              return Err("Error finding digit");
            }
          }
        }
        None => {
          return Err("Error converting character to number");
        }
      }
    }
    // println!("current_char: {:?}", current_char.to_string());
    // println!("plus: {:?}", String::from(Plus.val().unwrap()));

    if current_char.to_string() == Plus.val().unwrap() {
      // println!("plus equals");
      let token = Some(Token::new_op(
        Add,
        self.op.get(&current_char.to_string()).unwrap().clone(),
      ));
      self.last_op = Plus;
      self.pos += 1;
      return Ok(token);
    } else {
      self.pos += 1;
      return Ok(Some(Token::new_op(Add, NoOp)));
    }
  }

  fn eat(&mut self, token_type: TokenType) -> Result<Option<i128>, &'a str> {
    match self.current_token.clone() {
      Ok(token) => match token {
        Some(token) => {
          if token.typ.val() == token_type.val() {
            // println!("token types are the same");
            self.current_token = self.get_next_token();
            Ok(None)
          } else {
            // println!("token types are different");
            self.state += 1 % 2;
            Ok(None)
          }
        }

        None => Ok(None),
      },

      Err(err) => Err(err),
    }
  }

  pub fn expr(&mut self) -> Result<i128, &str> {
    {
      self.current_token = self.get_next_token();
    }

    let left = match self.current_token.clone() {
      Ok(val) => match val {
        Some(val) => match val.val {
          TokenValue::TokenNumValue(val) => val,

          TokenValue::TokenOpValue(val) => match val {
            TokenOpValue::Plus => NoNum,
            TokenOpValue::NoOp => NoNum,
          },
        },

        None => NoNum,
      },

      Err(err) => {
        return Err(err);
      }
    };

    {
      loop {
        if self.state == 0 {
          self.eat(Integer)?;
        } else {
          break;
        }
      }
    }

    let op = match self.current_token.clone() {
      Ok(token) => match token {
        Some(token) => match token.clone().val {
          TokenValue::TokenOpValue(_) => token,

          TokenValue::TokenNumValue(val) => match val {
            _ => token,
          },
        },

        None => Token::new_num(Integer, NoNum),
      },

      Err(err) => {
        return Err(err);
      }
    };

    {
      loop {
        if self.state == 1 {
          self.eat(Add)?;
        } else {
          break;
        }
      }
    }

    let right = match self.current_token.clone() {
      Ok(val) => match val {
        Some(val) => match val.val {
          TokenValue::TokenNumValue(val) => val,

          TokenValue::TokenOpValue(val) => match val {
            TokenOpValue::Plus => NoNum,
            TokenOpValue::NoOp => NoNum,
          },
        },

        None => NoNum,
      },

      Err(err) => {
        return Err(err);
      }
    };

    {
      loop {
        if self.state == 0 {
          self.eat(Integer)?;
        } else {
          break;
        }
      }
    }

    let left = left.val().unwrap_or(0);
    let right = right.val().unwrap_or(0);

    match op.typ {
      _ => {
        if self.last_op.val() == Plus.val() {
          Ok((left + right) as i128)
        } else {
          Err("Error: Unknown operator")
        }
      }
    }
  }
}
