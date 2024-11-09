use std::io;
fn main() {
  let mut input1= String::new();
  let mut input2 = String::new();

  println!("\nWhat is your name?");
  io::stdin().read_line(&mut input1).expect("No name given");
  println!("\nname:{}", input1 );

  println!("What is your age?");
  io::stdin().read_line(&mut input2).expect("no age given");
  let age: u32=input2.trim().parse().expect("Not a number"); 
  if age >= 18  {
  	println!("\nWelcome In");
  }else {
  println!("\nSorry, you are not old enough to enter");
  }
}
