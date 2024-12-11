use std::io::Read;

fn main() {

let m = read("commisioner.txt");
let  l =read("ministry.txt");
let b = read("geopolitical_zone.txt");

let commisioner: Vec<&str> = m.split("\n").collect();
let ministry: Vec<&str> = l.split("\n").collect();
let geopolitical_zone: Vec<&str> = b.split("\n").collect();

 for i in 0..commisioner.len() {
     println!("{}, {}, {}", commisioner[i].trim(), ministry[i].trim(), geopolitical_zone[i].trim() );


   }

}

fn read(a: &str) -> String {
	let mut file = std::fs::File::open(a).unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	return contents.trim().to_string();
}
