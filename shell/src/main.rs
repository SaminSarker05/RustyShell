use std::io::stdin;
use std::io::{self, Write};
use std::process::Command;

fn main() {
  loop {
    print!("> ");
    let _ = io::stdout().flush();

    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();

    let mut line = _input.trim().split_whitespace();
    let command = line.next().unwrap();
    let arguments = line;

    

    let mut child = Command::new(command)
      .args(arguments)
      .spawn()
      .unwrap();

    child.wait();
  }

}
