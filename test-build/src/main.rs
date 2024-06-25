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

fn main() {
    // let mut x = 5;
    println!("Hello, world!");

    // specify type after variable name
    let _a: bool = true;

    // cargo build
}
