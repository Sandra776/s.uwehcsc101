use std::io;
fn main() {
  let mut input1= String::new();
  println!("Please enter a number");
  io::stdin().read_line(&mut input1).expect("no valid number");
  let mut num: i32= input1.trim().parse().expect("no valid number");

  while num <10 {
  	num+=1;
  	println!("inside the loop num = {}",num );
  }
println!("outside the loop num ={}",num );
}
