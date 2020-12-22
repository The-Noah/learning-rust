/**
 * Vectors - resizable arrays
 */

use std::mem;

pub fn run(){
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // re-assign value
  numbers[2] = 20;

  // add on to vector
  numbers.push(5);
  numbers.push(6);

  // pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // get single value
  println!("Single Value: {}", numbers[0]);

  // get vector length
  println!("Vector Length: {}", numbers.len());

  // vectors are heap allocated
  println!("Vector Occupies {} bytes", mem::size_of_val(&numbers));

  // get slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // loop through vector values
  for x in numbers.iter(){
    println!("Number: {}", x);
  }

  // loop and mutate values
  for x in numbers.iter_mut(){
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);
}
