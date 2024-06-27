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

    let line = _input.trim();
    if line.is_empty() { continue; }

    let cmds: Vec<&str> = line.split('|')
      .map(|cmd| cmd.trim())
      .collect();
    
    if cmds.len() == 1 { 
      execute_command(cmds[0]); 
    } else {
      execute_pipe(cmds);
    }

    
    // let command = line.next().unwrap();
    // let args: Vec<&str> = line.collect();

    // match command {
    //   "cd" => {
    //     if args.is_empty() {
    //       println!("cd: missing arg");
          
    //     } else {
    //       let new_dir = args[0];
    //       let result = env::set_current_dir(new_dir);
    //       if let Err(e) = result {
    //         println!("{}", e);
    //       }
    //     }
    //   },
    //   "exit" => { break; }
    //   _ => {
    //     let child = Command::new(command)
    //       .args(args)
    //       .spawn();
        
    //     // check that command is valid
    //     match child {
    //       Ok(mut child) => { let _ = child.wait(); }
    //       Err(e) => { println!("{}", e); }
    //     }
    //   }
    // }

  }

}


fn execute_command(command: &str) -> () {
  let mut line = command.split_whitespace();
  let cmd = line.next().unwrap();
  let args: Vec<&str> = line.collect();

  match cmd {
    "cd" => {
      if args.is_empty() {
        println!("cd: missing arg");
      } else {
        let new_dir = args[0];
        let result = env::set_current_dir(new_dir);
        if let Err(e) = result { println!("{}", e); }
      }
    }
    "exit" => { std::process::exit(0); }
    _ => {
      let child = Command::new(cmd)
        .args(args)
        .spawn();
      match child {
        Ok(mut child) => {
          let _ = child.wait();
        }
        Err(e) => { println!("{}", e); }
      }
    }
  }
}
