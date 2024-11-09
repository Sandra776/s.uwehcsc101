fn main() {
  let mut x = 0;
  loop {
  	x+=1;
  	println!("x={}", x);
  if x==15{
  	break;
  }}



	let mut count = 0;
	for num in 1 ..= 21{
		if num > 10{println!("\n{}",num );
		continue}
		count+=1
	}
 println!("\nThe count value of numbers greater than 10 (between 1 and 20) is: {:?}", count );
}