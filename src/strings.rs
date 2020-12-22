/**
 * primitive str = immutable fixed-length string somewhere in memory
 * String = growable, heap-allocated data structure - use when you need to modify
 */

pub fn run(){
  let mut hello = String::from("Hello ");

  // get length (works on both types)
  println!("Length: {}", hello.len());

  // push on a char
  hello.push('W');

  // push on a string
  hello.push_str("orld!");

  // capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // check if empty
  println!("Is Empty: {}", hello.is_empty());

  // contains
  println!("Contains 'World' {}", hello.contains("World"));

  // replace
  println!("Replace: {}", hello.replace("World", "There"));

  // loop through string by whitespace
  for word in hello.split_whitespace(){
    println!("{}", word);
  }

  // create string with specific capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{}", s);
}
