fn main() {
 let name = "Aisha lawal";
 let uni :&str = "Pan-Atlantic University";
 let addr :&str = "km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
 println!("Name: {}", name );
 println!("University: {}, \nAddress: {}", uni, addr );

 let department:&'static str = "Computer Science";
 let school: &'static str = "School of Science and Technology";
 println!("Department: {}, \nSchool: {}", department, school );
}
