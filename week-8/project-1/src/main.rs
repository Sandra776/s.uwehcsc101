use std::io::Write;


fn main() {
	let mut lager = Vec::<String>::new();
	let mut stout = Vec::<String>::new();
	let mut non_alcoholic = Vec::<String>::new();
 

   let m:u32 =  input("How many drinks do you want to input?").parse().expect("fail");

   for _i in 1..=m {
       let a = input("Please input the type of drink you want (lager, stout or non-alcoholic)");
       let b = input("Please input the drink you want");

       
    if a == "lager" {
    	lager.push(b);
    }else if a == "stout"{
        stout.push(b);
    } else if a == "non-alcoholic"{
    	non_alcoholic.push(b);
    }

   }
   
     
     
     let mut filelager = std::fs::File::create("datalager.txt").expect("create failed");
     filelager.write_all(lager.join("\n").as_bytes()).expect("Write failed");
	
     let mut filestout = std::fs::File::create("datastout.txt").expect("create failed");
     filestout.write_all(stout.join("\n").as_bytes()).expect("Write failed");
    
     let mut filenon_alcoholic = std::fs::File::create("datanon_alcholic.txt").expect("create failed");	
     filenon_alcoholic.write_all(non_alcoholic.join("\n").as_bytes()).expect("Write failed");

     println!("Data has been saved");

}


fn input (a: &str) ->String {
	println!("{}", a);
	let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("fail");
    return input1.trim().to_string();
}

