struct Rectangle {
	width:u32,
	length:u32,
}

impl Rectangle {
	fn area (&self)-> u32{
		self.width*self.length
	}
}

fn main() {
  let small = Rectangle{
  	width:10,
  	length:20,
  };

println!("width is {} \n height is {} \n area of Rectangle is {}", small.width, small.length, small.area());  
}
