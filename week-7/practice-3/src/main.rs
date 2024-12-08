
fn value (n:Option<&char>){
	println!("element of vector {:?}", n);
}

fn main() {
let v = vec!['r','u','s','t','a','c','i','a','n',];

let mut input1 = String::new();
println!("\nenter an index value (0-8)");
std::io::stdin().read_line(&mut input1).expect("fail");

let index:usize = input1.trim().parse().expect("invalid input");

let ch: Option<&char> = v.get(index);
value(ch);
}
