use std::io::Write;


fn main() {
	let mut name = vec!["Oluchi Mordi", "Adams Aliyu", "shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
	let mut matric_number = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
	let mut department = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
  let mut level = vec!["300","100","200","200","100"];
 

     let mut file= std::fs::File::create("data.txt").expect("create failed");


   for i in 0..name.len() {
     println!("{}, {}, {}, {}",name[i],matric_number[i], department[i], level[i] );
  
     let line = format!("{}, {}, {}, {}\n",name[i],matric_number[i], department[i], level[i]);
     file.write_all(line.as_bytes()).expect("Write failed");


   }
   
     
     
	
     
     println!("Data has been saved");

}


