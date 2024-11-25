use std::io;

fn main() {
	loop {
		
	
println!("Welcome to my E-calculator,\n \nThis program is optimized to help you find the answers to various mensuration questions");
println!("\nPlease select the number representing the formula you want from the array below");

let array_formula = ["1 = Area of Trapezium", "2 = Area of rhombus", "3 = Area of Parallelogram", " 4 = Area of Cube", "5 = Volume of cylinder"];
println!("\n{:?}", array_formula);
let mut input = String::new();
io::stdin().read_line(&mut input).expect("not valid");
let a: i32 = input.trim().parse().expect(".");

if a == 1 {
	trapezium();
} else if a == 2 {
    rhombus();
}else if a == 3{
	parallel();
}else if a == 4{
	cube();
}else if a == 5 {
	cylinder();
}else {
	println!("\nNot a valid formula");
};

println!("\nWould you like to make another calculaton? (choose true or false)");
let mut input2 = String::new();
io::stdin().read_line(&mut input2).expect("m");
let b: bool = input2.trim().parse().expect("invalid");

if b == false {
	println!("Thankyou for your patronage, \nI hope you found it useful.");
	break;
};
}
}
 

 //formula functions

 //TRAPEZIUM
 fn trapezium(){
 println!("Please insert your value for base1");
 let mut t1 = String::new();
io::stdin().read_line(&mut t1).expect("m");
let c: f32 = t1.trim().parse().expect("invalid");

 println!("Please insert your value for base2");
 let mut t2 = String::new();
io::stdin().read_line(&mut t2).expect("m");
let d: f32 = t2.trim().parse().expect("invalid");

println!("Please insert your value for height");
 let mut t3 = String::new();
io::stdin().read_line(&mut t3).expect("m");
let e: f32 = t3.trim().parse().expect("invalid");
 
let ar1: f32 = (e/2.0)*(c+d);
println!("\nArea of trapezium = {}", ar1 );
 }

 //RHOMBUS

 fn rhombus(){
println!("Please insert your value for diagonal 1");
let mut r1 = String::new();
io::stdin().read_line(&mut r1).expect("m");
let f: f32 = r1.trim().parse().expect("invalid");


println!("Please insert your value for diagonal 2");
let mut r2 = String::new();
io::stdin().read_line(&mut r2).expect("m");
let g: f32 = r2.trim().parse().expect("invalid");

let ar2: f32 = 0.5*f*g;
println!("\nThe Area of your Rhombus = {}", ar2);
 }


//PARALLELOGRAM
 fn parallel(){
println!("Please insert your value for base ");
let mut p1 = String::new();
io::stdin().read_line(&mut p1).expect("m");
let h: f32 = p1.trim().parse().expect("invalid");

println!("Please insert your value for Altitude");
let mut p2 = String::new();
io::stdin().read_line(&mut p2).expect("m");
let i: f32 = p2.trim().parse().expect("invalid");

let ar3: f32 = i*h;
println!("The area of the Parallelogram = {}", ar3);

}


//CUBE
fn cube(){
println!("Please insert your value for side");
let mut c1 = String::new();
io::stdin().read_line(&mut c1).expect("m");
let j: f32 = c1.trim().parse().expect("invalid");

let ar4: f32 = 6.00*j*j;
println!("The area of your cube = {}", ar4 );
 }


//CYLINDER
 fn cylinder(){
println!("Please insert your value for radius");
let mut cy1 = String::new();
io::stdin().read_line(&mut cy1).expect("m");
let k: f32 = cy1.trim().parse().expect("invalid");

println!("Please insert your value for height");
let mut cy2 = String::new();
io::stdin().read_line(&mut cy2).expect("m");
let l: f32 = cy2.trim().parse().expect("invalid");
 
let ar5: f32 = (22.0/7.0)*k*k*l;
println!("The volume of your cylinder = {}", ar5);
 }