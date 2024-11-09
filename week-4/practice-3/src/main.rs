use std::io;
fn main() {
   let mut input1= String::new();
   let mut input2 = String::new();

   println!("\nPlease input value for base");
   io::stdin().read_line(&mut input1).expect("No value inputed");
   let base: f32= input1.trim().parse().expect("Value not a float");

    println!("\nPlease input value for base");
   io::stdin().read_line(&mut input2).expect("No value inputed");
   let height: f32= input2.trim().parse().expect("Value not a float");

   if base> 0.0 {
       let area= (0.5)* base* height;
       println!("Your area is:{}", area );
   }
}
