use std::io;
fn add(a: i32, b: i32){
	let sum = a + b;
	println!("Th sum of A and B is {}", sum);
}

fn main() {
   let mut input1 = String::new();
   println!("Enter Input for parameter A");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let a: i32 = input1.trim().parse().expect("Failed input");

    let mut input2 = String::new();
   println!("Enter Input for parameter B");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let b: i32 = input2.trim().parse().expect("Failed input");

   add(a,b);
   }
