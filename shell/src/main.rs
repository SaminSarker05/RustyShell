use std::io::stdin;
use std::io::{self, Write};
use std::process::{Command, Stdio};
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

// OPTION<> enum type represnts value that is or not present; SOME or None
// RESULT<> enum type represents a sucess or failure
// SOME<> enum part of option 

// UNWRAP() works on option or result to extract value, if none or err panics
// TAKE makes option enum empty



fn execute_pipe(commands: Vec<&str>) {
  let mut prev_cmd: Option<Command> = None;
  let mut first_cmd: bool = true;

  for cmd in commands {
    let mut line = cmd.trim().split_whitespace();
    let cmd = line.next().unwrap();
    let args: Vec<&str> = line.collect();

    let mut child_process = Command::new(cmd)
      .args(args)
      .stdin(Stdio::piped());
    
    if let Some(mut prev) = prev_cmd.take() {
      child_process.stdin(prev.stdout(Stdio::piped()).unwrap());
    }
  }
}
