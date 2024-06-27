use std::io::stdin;
use std::io::{self, Write};
use std::process::Command;

fn main() {
  loop {
    print!("> ");
    let _ = io::stdout().flush();

    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();

    let command = _input.trim();
    
    let mut child = Command::new(command)
      .spawn()
      .unwrap();

    child.wait();
  }

}
