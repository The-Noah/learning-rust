pub fn run(){
  let name = "Noah";
  let mut age = 17;

  println!("My name is {} and I am {}", name, age);

  age = 18;
  println!("My name is {} and I am {}", name, age);

  // define a constant
  const ID: i32 = 1;
  println!("ID: {}", ID);

  // assign multiple variables at once
  let (my_name, my_age) = ("Noah", 17);
  println!("{} is {}", my_name, my_age);
}
