use std::io;
fn main() {
   println!("\nStudent Information Management System");

   //input name
   println!("\nPlease Enter name.");
   let mut name = String::new();
   io::stdin()
   .read_line(&mut name)
   .expect("Failed to read input");
   println!("Your name is:{}", name);

   //input age
   println!("\nEnter Age.");
   let mut age=String::new();
   io::stdin()
   .read_line(&mut age).expect("Failed to read input");
    let age:u32 = age.trim().parse().expect("Value is not an integer");
    println!("Your age is:{}", age );
}
