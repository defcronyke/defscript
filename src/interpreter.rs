use crate::token::{Token, TokenNumValue, TokenOpValue, TokenType, TokenValue};
use TokenOpValue::*;
use TokenType::*;

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
  pub fn expr(&mut self) -> Result<TokenNumValue, String> {
    self.current_token = self.get_next_token()?;

    let left = self.current_token.clone().unwrap();
    self.eat(Integer)?;

    let op = self.current_token.clone().unwrap();
    if op.typ == Add {
      self.eat(Add)?;
    } else if op.typ == Subtract {
      self.eat(Subtract)?;
    } else {
      return Err("Unknown operator type".into());
    }

    let right = self.current_token.clone().unwrap();
    self.eat(Integer)?;

    if op.typ == Add {
      let left = match left.val {
        TokenValue::TokenNumValue(num) => num,
        _ => 0,
      };

      let right = match right.val {
        TokenValue::TokenNumValue(num) => num,
        _ => 0,
      };

      Ok(left + right)
    } else if op.typ == Subtract {
      let left = match left.val {
        TokenValue::TokenNumValue(num) => num,
        _ => 0,
      };

      let right = match right.val {
        TokenValue::TokenNumValue(num) => num,
        _ => 0,
      };

      Ok(left - right)
    } else {
      Err("Unknown operator".into())
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

          if current_char.to_string() == Plus.val() {
            self.advance();
            return Ok(Some(Token::new_op(Add, Plus)));
          }

          if current_char.to_string() == Minus.val() {
            self.advance();
            return Ok(Some(Token::new_op(Subtract, Minus)));
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
