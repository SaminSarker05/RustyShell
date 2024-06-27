// importing standard input module
use std::io::stdin;

fn main() {
  // indefinte loop 
  loop {
    print!("> ");
    let 

  }
  // declared a mutable string variable
  let mut _userinput = String::new();
  stdin().read_line(&mut _userinput).unwrap();  // reads keyboard input into string

  let command = _userinput.trim();  // remove newline

  println!("{}", _userinput);
}
