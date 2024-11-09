use std::io;

fn main() {
let mut input1= String::new();
let mut input2= String::new();
let mut input3= String::new();

println!("Input value for a");
io::stdin().read_line(&mut input1).expect("invalid character");
let a:i32= input1.trim().parse().expect("invalid character");

println!("Input value for b");
io::stdin().read_line(&mut input2).expect("invalid character");
let b:i32= input2.trim().parse().expect("invalid character");

println!("Input value for c");
io::stdin().read_line(&mut input3).expect("invalid character");
let c:i32= input3.trim().parse().expect("invalid character");


let  d: f32= ((b.pow(2))-(4*a*c)) as f32;

println!("\nyour discriminant is = {}",d );

if d < 0.0 {
	println!("\nyour equation has no real roots!!");
}else if d == 0.0{
	println!("\nyour equation has one real root");
	let x: i32=(-b)/(2*a);
	println!("\n the root of your equation is {}",x);
}else if d > 0.0{
	println!("\nYour equation has 2 real roots!!");
	let m= d.sqrt();
	let y1= ((-b) as f32+m)/(2.0*(a)as f32);
	let y2=((-b)as f32-m)/(2.0*(a) as f32);
	println!("The roots of your equation are y1= {} and y2={}", y1, y2 );
}
}
