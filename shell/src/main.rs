use std::io::stdin;

fn main() {
  let mut _input = String::new();
  stdin().read_line(&mut _input).unwrap();

  let command = _input.trim();
  println!("{}", command);

}
