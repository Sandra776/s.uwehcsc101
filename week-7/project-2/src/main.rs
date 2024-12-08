fn main() {
 
  println!("Welcome to Ernst & Young's experience checker");

  println!("\nHow many candidates are you checking?");
  let mut input3 = String::new();
  std::io::stdin().read_line(&mut input3).expect("fail");
  let n: u32 = input3.trim().parse().expect("fail");

  let mut devs: Vec<(String, u32,i32)> = Vec::new();
  let mut candno:i32 = -1;

// ADDING CANDIDATES
  for _x in 1..=n{

  let mut input1 = String::new();
  let mut input2 = String::new();

  println!("\nPlease input candidate name");
  std::io::stdin().read_line(&mut input1).expect("fail");
  let name: String = input1.trim().parse().expect("fail");
  
  println!("Please input Years of experience (enter Whole number)");
  std::io::stdin().read_line(&mut input2).expect("fail");
  let exp: u32 = input2.trim().parse().expect("fail");

  candno+=1;
  let dev: (String, u32,i32) = (name, exp, candno); 
  devs.push(dev);
  
 }
  println!("Data succesfully saved");
  println!("\n{:?}", devs);

//EDITING CANDIDATE LIST
 loop {	
  let mut input4 = String::new();
  println!("would you like to remove, change or add a candidate? (choose 1=remove, 2=change, 3=add, 4=No i would like to continue)");
  std::io::stdin().read_line(&mut input4).expect("fail");
  let by: u32 = input4.trim().parse().expect("fail");


 if by == 4{
  break;
 }
 else{
  let mut input8 = String::new();
  println!("How many candidates would you like to remove, change or add?");
  std::io::stdin().read_line(&mut input8).expect("fail");
  let l: u32 = input8.trim().parse().expect("fail");

  for _g in 1..=l{

//REMOVING CANDIDATES
   if by == 1 {
   let mut input5 = String::new();
   println!("which candidate would you like to remove (choose the number after the candidate name and experience)");
   std::io::stdin().read_line(&mut input5).expect("fail");
   let no: usize = input5.trim().parse().expect("fail");
  
   println!("Candidate {:?} has been removed", devs[no]); 
   devs.remove(no); 

   for (index, dev) in devs.iter_mut().enumerate() {
   dev.2 = index as i32;
    }

   println!("Your new candidate list is {:?}",devs );
   
   }else
   
//CHANGING CANDIDATES
   if by ==2{
   let mut inputnm = String::new();
   println!("which candidate would you like to replace (choose the number after the candidate name and experience)");
   std::io::stdin().read_line(&mut inputnm).expect("fail");
   let nm: usize = inputnm.trim().parse().expect("fail");
    
   let mut inputny = String::new();
   println!("\nPlease input candidate name");
   std::io::stdin().read_line(&mut inputny).expect("fail");
   let name2: String = inputny.trim().parse().expect("fail");
  
   let mut inputnx = String::new();
   println!("Please input Years of experience (enter number)");
   std::io::stdin().read_line(&mut inputnx).expect("fail");
   let exp2: u32 = inputnx.trim().parse().expect("fail");
     
   devs[nm] = (name2,exp2, nm.try_into().unwrap());
      
   println!("Your new candidate list is {:?}",devs );
   
   }else

//ADDING EXTRA CANDIDATES
  if by == 3{
  let mut input6 = String::new();
  let mut input7 = String::new();
  

  println!("\nPlease input candidate name");
  std::io::stdin().read_line(&mut input6).expect("fail");
  let name1: String = input6.trim().parse().expect("fail");
  
  println!("Please input Years of experience (enter number)");
  std::io::stdin().read_line(&mut input7).expect("fail");
  let exp1: u32 = input7.trim().parse().expect("fail");
  candno+=1;

  let dev1: (String, u32,i32) = (name1, exp1, candno); 
  devs.push(dev1);
  println!("Your new candidate list is {:?}",devs );
   }
  }
 }
 }

//RANKING CANDIDATES
  devs.sort_by(|a, b| b.1.cmp(&a.1));
  println!("\nThe candidate with the highest years of experience is :{:?}",devs[0]);
}                                                                                                                                                                                                                                                                                                          


