use std::io::stdin;
use std::io::{self, Write};
use std::process::Command;
use std::env; // environment module
// std = standard library

fn main() {
  loop {
    print!("> ");
    let _ = io::stdout().flush();

    let mut _input = String::new();
    stdin().read_line(&mut _input).unwrap();

    let mut line = _input.trim().split_whitespace();
    let command = line.next().unwrap();
    let args: Vec<&str> = line.collect();

    match command {
      "cd" => {
        if args.is_empty() {
          println!("cd: missing arg");
          
        } else {
          let new_dir = args[0];
          let result = env::set_current_dir(new_dir);
          if let Err(e) = result {
            println!("{}", e);
          }
        }
      },
      "exit" => { break; }
      _ => {
        let child = Command::new(command)
          .args(args)
          .spawn();
        
        // check that command is valid
        match child {
          Ok(mut child) => { let _ = child.wait(); }
          Err(e) => { println!("{}", e); }
        }
      }
    }

  }

}
