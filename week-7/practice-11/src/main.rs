fn main() {
 let numbers = [1,2,3,4,5];
 println!("Original array = {:?}", numbers);

 let slice1 = &numbers[1..3];
 println!("slice1 elements {:?}", slice1 );


 let slice2 = &numbers[..3];
 println!("slice2 elements {:?}", slice2 );


 let slice3 = &numbers[2..];
 println!("slice3 elements {:?}", slice3 );


 let slice4 = &numbers[..];
 println!("slice4 elements {:?}", slice4 );


}
