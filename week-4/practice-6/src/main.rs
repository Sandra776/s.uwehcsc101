use std::io;
fn main() {
	let mut input1 = String:: new();
	let mut input2 = String:: new();

  println!("Insert lower bound");
  io::stdin().read_line(&mut input1).expect("Invalid number");
  let lower: i32= input1.trim().parse().expect("Invalid number");

  println!("Insert upper bound");
  io::stdin().read_line(&mut input2).expect("Invalid number");
  let upper: i32= input2.trim().parse().expect("Invalid number");
for  x in lower ..= upper {
   let a: i32 = x;
   let b: i32 = x+ upper;
   let s: f32 = (a*b- a/b)as f32;
   println!("The answer is: {}",s );	
}
}
