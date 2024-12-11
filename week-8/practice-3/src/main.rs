use std::fs;

fn main() {
  fs::remove_file("data.txt").expect("COULD NOT REMOVE FILE");
  println!("file is removed");
}
