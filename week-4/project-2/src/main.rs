use std:: io;
fn main() {
 //Variable assignment
   let mut name= String::new();
   let mut input2 = String::new();
   let mut input3= String::new();

//Name
   println!("What is your name?");
   io::stdin().read_line(&mut name).expect("Nothing inputted");

//Experience
   println!("Are you experienced(true/false)");
   io::stdin().read_line(&mut input2).expect("nothing inputed");
   let exp: bool =input2.trim().parse().expect("pick true or false!!");
  
//Age
   println!("What is your age?");
   io::stdin().read_line(&mut input3).expect("no valid input");
   let age: u32= input3.trim().parse().expect("not a valid age");
  

 println!("name: {}",name );
 println!("Experienced:{}", exp );
 println!("Age: {}",age );

//If statement
  if exp== true  && age >= 40{
  	println!("Your incentive: N1_560_000");
  }else if exp== true && age >= 30 && age < 40{
    println!("Your incentive: N1_480_000");
  }else if exp == true && age < 28 {
  	println!("Your incentive: N1_300_000");
  }else if exp == false{
  	println!("Your incentive: N100_000");
  }
}
