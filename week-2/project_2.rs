 fn main() {
 	//amount
	let t: f64 = 450000.00;
	let m : f64 = 1500000.00;
	let h : f64 = 750000.00;
	let d : f64 = 2850000.00;
	let a : f64 = 250000.00;

	//quantity
	let tq : f64 = 2.00;
	let mq : f64 = 1.00;
	let hq : f64 = 3.00;
	let dq : f64 = 3.00;
	let aq : f64 = 1.00;

	//sum
	let suma = t*tq + m*mq + h*hq + d*dq + a*aq;
	println!("this is the sum {}", suma );

	let sumq = tq + mq+ hq + dq + aq;

	//average
	let av = suma/sumq;
	println!("the average is {}", av );
}