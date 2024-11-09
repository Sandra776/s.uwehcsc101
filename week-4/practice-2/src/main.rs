use std::io;
fn main() {
	println!("Calculator for area of a triangle");
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
println!("\nPlease insert the first value for side");
    io::stdin().read_line(&mut input1).expect("not a number");
    let a: f32= input1.trim().parse().expect("Value is not a float");

 println!("\nPlease insert the second value for side");
    io::stdin().read_line(&mut input2).expect("not a number");
    let b: f32= input2.trim().parse().expect("Value is not a float");

 println!("\nPlease insert the third value for side");
    io::stdin().read_line(&mut input3).expect("not a number");
    let c: f32= input3.trim().parse().expect("Value is not a float");

   let s:f32= (a+b+c)/2.0;
   let mut ar= s*(s-a)*(s-b)*(s-c);
   ar = ar.sqrt();
   println!("\nThe area of your triangle is:{}",ar );
}

