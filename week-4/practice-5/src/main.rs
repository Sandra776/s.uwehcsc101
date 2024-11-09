use std::io;
fn main() {
   let mut input1= String::new();

   println!("\nPlease put your height (in centimetres)");
   io::stdin().read_line(&mut input1).expect("Not a valid number");
   let height: f32= input1.trim().parse().expect("Not a float");
   println!("\nHeight: {}cm", height );

   if height >=150.0 && height <=170.0 {
     println!("\nYou are average height"); 
   } else if height >170.0 && height <=195.0 {
   	println!("\nYou are Tall");
   }else if height <150.0 && height >100.0 {
   	println!("\nYou are a dwarf");
   }else {
   	println!("You are an abnormal height");
   }
}
