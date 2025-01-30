use std::io;
use std::io::Read; 

fn main() {
	let p = (1,"employee",2,"administrator",3,"project Manager",4,"Vendor",5,"Costumer");
	println!("Welcome to the globacom datbase\n Please pick the number beside your position from the list below");
	println!("{:?}", p);

    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("not valid");
    let a: i32 = input.trim().parse().expect(".");

  if a == p.0{
   read("staff_tb.sql"); 
   }else if a == p.2{
   	read("globacom_dbase.sql");
   }else if a == p.4{
   	read("project.sql");
   }else if a == p.6{
   	read("dataplan.sql");
   }else if a == p.8{
   	read("customer.sql");
   }else{
   	println!("Invalid Input");
   }
}


fn read(a: &str) -> String {
	let mut file = std::fs::File::open(a).unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	println!("{}", contents);
	return contents.trim().to_string();

}