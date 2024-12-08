fn main() {
   println!("Welcome to the Federal goverment's\nPublic Service APS Level checker");
   
   println!("Please input name");
   let mut input3= String::new();
   std::io::stdin().read_line(&mut input3).expect("fail");
   let n = input3;

   println!("Please input Years of experience");
   let mut input1= String::new();
   std::io::stdin().read_line(&mut input1).expect("fail");
   let e: f32 = input1.trim().parse().expect("fail");

  
   let exp = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
   let jobs = (1,"Office Administrator",2, " Academic", 3," Lawyer",4, "Teacher");

   println!("please choose your proffesion from the list below (Choose the number beside your proffesion)");

   println!("{:?}",jobs );
   let mut input2= String::new();
   std::io::stdin().read_line(&mut input2).expect("fail");
   let l: u32 = input2.trim().parse().expect("fail");

   if l== 1{
   office_admin(&exp,e,n);
   }else if l== 2{
   academic(&exp,e,n);
   }else if l== 3{
  	lawyer(&exp,e,n);
   }else if l== 4 {
  	teacher(&exp,e,n);
   } 
}

fn office_admin (exp:&Vec<&str>,e:f32,n:String) {
   let oa =(1,"Intern",2,"Administrator",3,"Senior Administrator",4,"Office Manager",5,"Director",6,"CEO");
   println!("{:?}", oa);
   println!("Please select level from the list above (Choose the number beside your level)");
   let mut input2= String::new();
   std::io::stdin().read_line(&mut input2).expect("fail");
   let l: u32 = input2.trim().parse().expect("fail");
 
   println!("\nName: {} ", n );
   println!("Years of experience: {}", e);

   if l ==1 && e>= 1.0 && e <=2.9{
	 println!("You are an {:?} with APS level {:?}", oa.1,exp[0] );
   }else if l==2 && e>= 3.0 && e <=5.0{
		println!("You are an {:?} with APS level {:?}", oa.3,exp[1] );

   }else if l==3 && e>= 5.1 && e <=8.0{
 		println!("You are a {:?} with APS level {:?}", oa.5,exp[2] );

   }else if l==4 && e>= 8.1 && e <=10.0{
		println!("You are an {:?} with APS level {:?}", oa.7,exp[3] );

   }else if l==5 && e>= 10.1 && e <=13.0{
		println!("You are a {:?} with APS level {:?}", oa.9,exp[4] );
 
   }else if l==6 && e>= 13.1 {
		println!("You are a {:?} with APS level {:?}", oa.11,exp[5] );

   }else{
	 println!("No valid APS level");
   }
}

fn academic (exp:&Vec<&str>,e:f32,n:String) {
   let ac = (1,"Research Assistant",2,"PhD Candidate",3,"Post-Doc Researcher",4,"Senior Lecturer",5,"Dean");
   println!("{:?}", ac);
    println!("Please select level from the list above (Choose the number beside your level)");
   let mut input2= String::new();
   std::io::stdin().read_line(&mut input2).expect("fail");
   let l: u32 = input2.trim().parse().expect("fail");
 
   println!("\nName: {} ", n );
   println!("Years of experience: {}", e);

   if l==1 && e>= 3.0 && e <=5.0{
		println!("You are a {:?} with APS level {:?}", ac.1,exp[1] );

   }else if l==2 && e>= 5.1 && e <=8.0{
		println!("You are a {:?} with APS level {:?}", ac.3,exp[2] );

   }else if l==3 && e>= 8.1 && e <=10.0{
		println!("You are a {:?} with APS level {:?}", ac.5,exp[3] );

    }else if l==4 && e>= 10.1 && e <=13.0{
		println!("You are a {:?} with APS level {:?}", ac.7,exp[4] );

    }else if l==5 && e>= 13.1 {
		println!("You are a {:?} with APS level {:?}", ac.9,exp[5] );

    }else{
	  println!("No valid APS level");
    }
}

fn lawyer (exp:&Vec<&str>,e:f32,n:String) {
   let la = (1,"Paralegal",2,"Junior Associate", 3,"Associate",4, "Senior Associate 1-2",5,"Senior Associate 3-4",6,"Partner");
    println!("{:?}", la);
   println!("Please select level from the list above (Choose the number beside your level)");
   let mut input2= String::new();
   std::io::stdin().read_line(&mut input2).expect("fail");
   let l: u32 = input2.trim().parse().expect("fail");
 
   println!("\nName: {} ", n );
   println!("Years of experience: {}", e);

   if l ==1 && e>= 1.0 && e <=2.9{
  	println!("You are a {:?} with APS level {:?}", la.1,exp[0] );
   }else if l==2 && e>= 3.0 && e <=5.0{
		println!("You are a {:?} with APS level {:?}", la.3,exp[1] );

   }else if l==3 && e>= 5.1 && e <=8.0{
		println!("You are an {:?} with APS level {:?}", la.5,exp[2] );

   }else if l==4 && e>= 8.1 && e <=10.0{
		println!("You are a {:?} with APS level {:?}", la.7,exp[3] );

   }else if l==5 && e>= 10.1 && e <=13.0{
		println!("You are a {:?} with APS level {:?}", la.9,exp[4] );

   }else if l==6 && e>= 13.1 {
		println!("You are a {:?} with APS level {:?}", la.11,exp[5] );
 
   }else{
  	println!("No valid APS level");
   }
}

fn teacher (exp:&Vec<&str>,e:f32,n:String) {
   let te = (1,"Placement",2,"Classroom Teacher",3,"Snr Teacher",4,"Leading Teacher",5,"Deputy Principal",6,"Principal");
   println!("{:?}", te);
    println!("Please select level from the list above (Choose the number beside your level)");
   let mut input2= String::new();
   std::io::stdin().read_line(&mut input2).expect("fail");
   let l: u32 = input2.trim().parse().expect("fail");
 
   println!("\nName: {} ", n );
   println!("Years of experience: {}", e);

    if l ==1 && e>= 1.0 && e <=2.9{
	 println!("You are an {:?} with APS level {:?}", te.1,exp[0] );
   }else if l==2 && e>= 3.0 && e <=5.0{
		println!("You are an {:?} with APS level {:?}", te.3,exp[1] );

   }else if l==3 && e>= 5.1 && e <=8.0{
		println!("You are a {:?} with APS level {:?}", te.5,exp[2] );

    }else if l==4 && e>= 8.1 && e <=10.0{
		println!("You are an {:?} with APS level {:?}",te.7,exp[3] );

    }else if l==5 && e>= 10.1 && e <=13.0{
		println!("You are a {:?} with APS level {:?}", te.9,exp[4] );

    }else if l==6 && e>= 13.1 {
		println!("You are a {:?} with APS level {:?}", te.11,exp[5] );

   }else{
  	println!("No valid APS level");
   }
}


