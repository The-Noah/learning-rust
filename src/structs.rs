/**
 * Structs - used to create custom data types
 */

// traditional struct
struct Color{
  red: u8,
  green: u8,
  blue: u8
}

// turple struct
struct TColor(u8, u8, u8);

struct Person{
  frist_name: String,
  last_name: String
}

impl Person{
  // construct person
  fn new(first: &str, last: &str) -> Person{
    Person{
      frist_name: first.to_string(),
      last_name: last.to_string()
    }
  }

  // get full name
  fn full_name(&self) -> String{
    format!("{} {}", self.frist_name, self.last_name)
  }

  // set last name
  fn set_last_name(&mut self, last: &str){
    self.last_name = last.to_string();
  }

  // name to tuple
  fn to_tuple(self) -> (String, String){
    (self.frist_name, self.last_name)
  }
}

pub fn run(){
  let mut c = Color{
    red: 255,
    green: 0,
    blue: 0
  };

  c.red = 200;

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  let mut tc = TColor(255, 0, 0);

  tc.0 = 200;

  println!("TColor: {} {} {}", tc.0, tc.1, tc.2);

  let mut p = Person::new("Marry", "Doe");
  println!("Person: {}", p.full_name());

  p.set_last_name("Williams");
  println!("Person: {}", p.full_name());

  println!("Person Tuple: {:?}", p.to_tuple());
}
