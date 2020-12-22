/**
 * Arrays - fixed list where elements are the same data types
 */

use std::mem;

pub fn run(){
  let mut numbers: [i32; 4] = [1, 2, 3, 4];

  // re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);

  // get single value
  println!("Single Value: {}", numbers[0]);

  // get array length
  println!("Array Length: {}", numbers.len());

  // arrays are stack allocated
  println!("Array Occupies {} bytes", mem::size_of_val(&numbers));

  // get slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
}
