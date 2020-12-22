mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;

fn main(){
  print::run();

  println!();
  vars::run();

  println!();
  types::run();

  println!();
  strings::run();

  println!();
  tuples::run();

  println!();
  arrays::run();

  println!();
  vectors::run();

  println!();
  conditionals::run();

  println!();
  loops::run();

  println!();
  functions::run();

  println!();
  pointer_ref::run();

  println!();
  structs::run();

  println!();
  enums::run();

  println!();
  cli::run();
}
