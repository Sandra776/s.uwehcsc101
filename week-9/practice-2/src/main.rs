fn main() {
let v = vec![10, 20, 30];
//let v2 = v;
display(v);
println!("In main {:?}", v);
}

fn display (v){
	println!("Inside display {:?}", v);
}