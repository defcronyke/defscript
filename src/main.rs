mod interpreter;
mod token;
use interpreter::Interpreter;
use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  // println!("{:?}", args);

  if args.len() < 3 {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
      match write!(&mut stdout, "> ") {
        Err(err) => println!("Error writing prompt to stdout: {}", err),
        _ => (),
      }

      match stdout.flush() {
        Err(err) => println!("Error flushing prompt to stdout: {}", err),
        _ => (),
      }

      let mut input = String::new();
      match stdin.read_line(&mut input) {
        Err(err) => println!("Error reading line from stdin: {}", err),
        _ => (),
      }

      let text = input.trim_end_matches('\n').trim_end_matches('\r');

      if input == "exit" {
        break;
      }

      let mut interpreter = Interpreter::new(text);

      let mut cont = false;

      let res = interpreter.expr().unwrap_or_else(|err| {
        println!("Error executing expression: {}", err);
        cont = true;
        1
      });

      if cont {
        continue;
      }

      match writeln!(&mut stdout, "{}", res) {
        Err(err) => println!("Error writing response to stdout: {}", err),
        _ => (),
      }
    }
  } else {
    let query = &args[1];

    if query == "-c" {
      let command = &args[2];

      let mut interpreter = Interpreter::new(command);

      let res = interpreter.expr().unwrap_or_else(|err| {
        println!("Error executing expression: {}", err);
        1
      });

      println!("{}", res);
    }
  }
}
