/*
cargo - rust package manager
specify project dependencies in cargo.toml
cargo automates buld and rus

cargo.toml = metadata/project dependencies
src/main.rs = main entry point for project

can create a binary or library project/crate
binary -> cargo new project_name
library (can be used as dependency) -> cargo new project_name --lib
cd project_name

cargo build = compiles rust code into machine executable
      - compiled artifact sotred in target/debug
cargo run = runs Rust project and executes the binary 
*/

use std::cmp::min;

fn main() {
  // 1. variables
  // immutable my default use mut to make mutable
  let x = 5;
  let mut y = 10;

  // println!("Hello, world!");
  // println!("{}, {}", x, y);

  // 2. statically typed must state type
  // type annotation
  let integer: i32 = 42;
  let float: f64 = 3.14;
  let boolean: bool = true;
  let character: char = 'R';

  // 3. tuple - immutable
  let tuple: (i32, f64, u8) = (500, 6.4, 1);

  // 4. array - making an 32 bit integer of size 4
  let array: [i32; 4] = [1, 2, 3, 4];

  add(32, 64);

  // 5. if else
  let number: i32 = 7;
  if number < 5 {
    println!("less than 5");
  } else if number == 5 {
    println!("equal to 5");
  } else {
    println!("greater than 5");
  }

  // 6. loop, while, for
  let mut count: i32 = 0;
  loop {
    count += 1;
    if count == 4 {
      break;
    }
  }
  let mut number: i32 = 3;
  while number != 0 {
    println!("{}", number);
    number -= 1;
  }

  let array: [i32; 4] = [10, 20, 30, 40];
  for element in array.iter() {
    println!("{}", element);
  }

  // 7. rust known for speed and safety
  // no garbage collector
  let s1 = String::from("hello");

  // vectors
  let mut v1 = Vec::new()
  v1.push(1);
}

// 8. functions defined with keyword fn
// define paramters and return type with arrow
// if void return type then dont specify
fn add(a: i32, b: i32) -> i32 {
  a + b
  // in a function the last statement or tail is whats returned
}

// 9. structs used for custom data types
struct User {
  username: String,
  email: String,
  active: bool,
}

// 10. matching
fn myMatch(n: i32) {
  match n {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),
  }// must be exhaustive
}

// use impl to add methods to structs

// 11. can make function/struct templates
struct myPair<T> {
  a: T,
  b: T,
}
