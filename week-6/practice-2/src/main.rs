use std::io;
fn checker(){
	let mut input = String::new();
	println!("Enter a character");
	io::stdin().read_line(&mut input).expect("Fsailed to enter character");
	let ch:i32 = input.trim().parse().expect("invalid input");

	if ch >= 0 && ch <= 9{
		println!(" character '{}' is a digit", ch );
	}else{
		println!("Character '{}' is not a digit", ch );
	}
}
fn main() {
	println!("Welcome to my function");
   checker();
}
