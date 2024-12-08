fn main() {
   let v = vec!['c','o','m','p','u','t','e','r'];
   let mut ipnut1 = String::new();

   println!("enter index value (0-7)");
   std::io::stdin().read_line(&mut ipnut1).expect("fail");
   let index:usize = ipnut1.trim().parse().expect("fail");

   let ch: char = v[index];

   print!("{} is the character for index [{}]\n",ch, index );
}
