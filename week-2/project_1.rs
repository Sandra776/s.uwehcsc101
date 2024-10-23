 fn main() {
let p: f64 = 520000000.00;
let r : f64 = 10.00;
let n: f64 = 5.00;

 // compound interest formula
let a= p*(1.00+(r/100.00)).powf(n);
println!("the amount is {}", a);
let ci= a-p;
println!("the compound interest after 5 years is {}", ci );
}