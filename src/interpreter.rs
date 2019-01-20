use crate::token::{Token, TokenNumValue, TokenOpValue, TokenType, TokenValue};
use TokenOpValue::*;
use TokenType::*;

pub type ExprResult = f64;

#[derive(Clone)]
pub struct Interpreter<'a> {
  text: &'a str,
  pos: usize,
  current_token: Option<Token>,
  current_char: Option<char>,
}

impl<'a> Interpreter<'a> {
  pub fn new(text: &'a str) -> Self {
    Self {
      text,
      pos: 0,
      current_token: None,
      current_char: Some(text.chars().nth(0).unwrap()),
    }
  }

  // Parser
  pub fn expr(&mut self) -> Result<ExprResult, String> {
    self.current_token = self.get_next_token()?;

    let left = self.current_token.clone().unwrap();
    self.eat(Integer)?;

    let op = self.current_token.clone().unwrap();
    match op.typ {
      Add => self.eat(Add)?,
      Subtract => self.eat(Subtract)?,
      Multiply => self.eat(Multiply)?,
      Divide => self.eat(Divide)?,
      _ => return Err("Unknown operator type".into()),
    }

    let right = self.current_token.clone().unwrap();
    self.eat(Integer)?;

    match op.typ {
      Add => {
        let left = match left.val {
          TokenValue::TokenNumValue(num) => num,
          _ => 0,
        };

        let right = match right.val {
          TokenValue::TokenNumValue(num) => num,
          _ => 0,
        };

        return Ok((left + right) as ExprResult);
      }

      Subtract => {
        let left = match left.val {
          TokenValue::TokenNumValue(num) => num,
          _ => 0,
        };

        let right = match right.val {
          TokenValue::TokenNumValue(num) => num,
          _ => 0,
        };

        return Ok((left - right) as ExprResult);
      }

      Multiply => {
        let left = match left.val {
          TokenValue::TokenNumValue(num) => num,
          _ => 0,
        };

        let right = match right.val {
          TokenValue::TokenNumValue(num) => num,
          _ => 0,
        };

        return Ok((left * right) as ExprResult);
      }

      Divide => {
        let left = match left.val {
          TokenValue::TokenNumValue(num) => num,
          _ => 0,
        };

        let right = match right.val {
          TokenValue::TokenNumValue(num) => num,
          _ => 0,
        };

        return Ok(left as ExprResult / right as ExprResult);
      }

      _ => return Err("Unknown operator".into()),
    }
  }

  fn advance(&mut self) {
    self.pos += 1;

    if self.pos > self.text.chars().count() - 1 {
      self.current_char = None;
    } else {
      self.current_char = Some(self.text.chars().nth(self.pos).unwrap());
    }
  }

  fn skip_whitespace(&mut self) {
    loop {
      match self.current_char {
        Some(current_char) => {
          if current_char.is_whitespace() {
            self.advance();
          } else {
            break;
          }
        }
        _ => (),
      }
    }
  }

  fn integer(&mut self) -> Result<TokenNumValue, String> {
    let mut res = String::new();
    loop {
      match self.current_char {
        Some(current_char) => {
          if current_char.is_digit(10) {
            res.push(current_char);
            self.advance();
          } else {
            break;
          }
        }
        None => break,
      }
    }

    match res.parse::<TokenNumValue>() {
      Ok(res) => Ok(res),
      Err(err) => Err(format!("Failed parsing integer: {}", err)),
    }
  }

  fn eat(&mut self, token_type: TokenType) -> Result<(), String> {
    match self.current_token.clone() {
      Some(current_token) => {
        if current_token.typ == token_type {
          self.current_token = self.get_next_token().unwrap();
        } else {
          return Err(format!("Failed eating token: Current token type does not match token type requested to be eaten: {} {}", current_token.typ, token_type));
        }
      }
      None => println!("Failed eating token: No current token"),
    }

    Ok(())
  }

  // Lexical analyzer
  fn get_next_token(&mut self) -> Result<Option<Token>, String> {
    loop {
      match self.current_char {
        Some(current_char) => {
          if current_char.is_whitespace() {
            self.skip_whitespace();
            continue;
          }

          if current_char.is_digit(10) {
            match self.integer() {
              Ok(num) => {
                return Ok(Some(Token::new_num(Integer, num)));
              }
              Err(err) => println!("Failed getting integer: {}", err),
            }
          }

          let current_char = current_char.to_string();

          if current_char == Plus.val() {
            self.advance();
            return Ok(Some(Token::new_op(Add, Plus)));
          }

          if current_char == Minus.val() {
            self.advance();
            return Ok(Some(Token::new_op(Subtract, Minus)));
          }

          if current_char == Asterisk.val() {
            self.advance();
            return Ok(Some(Token::new_op(Multiply, Asterisk)));
          }

          if current_char == Slash.val() {
            self.advance();
            return Ok(Some(Token::new_op(Divide, Slash)));
          }

          return Err("Failed getting next token".into());
        }
        None => {
          return Ok(Some(Token::new_op(Eof, NoOp)));
        }
      }
    }
  }
}
