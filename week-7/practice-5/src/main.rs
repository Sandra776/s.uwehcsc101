fn main() {
let mut city: Vec<String> = Vec::new();
 println!("the city vector has {} element(s)", city.len());

 let mut input1 = String::new();
 println!("How many cities do you want to enter");
 std::io::stdin().read_line(&mut input1).expect("fail");
 let city_num:i32 = input1.trim().parse().expect("fail");

 for count in 0..city_num {
 	let mut  input2 = String::new();
 	print!("Enter city {:?}\n", count+1);

   std::io::stdin().read_line(&mut input2).expect("fail");
   let new_city:String = input2.trim().parse().expect("fail");
   city.push(new_city);
 }

println!("Your preffered cities are :\n ");
let mut count=1;
for i in city {
	println!("{} {}",count,i );
	count+=1;
 }
}
