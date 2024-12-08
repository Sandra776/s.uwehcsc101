fn main() {
  let fullname = "Chidubem John Umeh";
  let deprtment = "Computer Science";
  let uni = "Pan Atlantic Uni";


  let mut school = "School of Science".to_string();
   school.push_str(" and technology");

    println!("The length of my fullname is: {}", fullname.len());
    println!("my fullname is {}", fullname);
    println!("I am a student of {} Department", deprtment);
    println!("{}", school );
    println!("{}", uni );
  }
