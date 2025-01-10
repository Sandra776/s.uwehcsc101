struct Laptop {
	name:String,
	price:u32,
	amount:u32
}

impl Laptop{
	fn tot (&self)->u32{
		self.price*self.amount
	}
}

fn main() {
  let hp = Laptop{
  	name:String::from("HP"),
  	price:650000/10,
  	amount:3
  };

  let ibm = Laptop{
  	name:String::from("IBM"),
  	price:755000/6,
  	amount:3
  };
  
    let toba = Laptop{
  	name:String::from("Toshiba"),
  	price:550000/10,
  	amount:3
  };

  let dell = Laptop{
  	name:String::from("Dell"),
  	price:850000/4,
  	amount:3
  };

    display(&hp.name,hp.tot()); 
    display(&ibm.name,ibm.tot());
    display(&toba.name,toba.tot());
    display(&dell.name,dell.tot());


  println!("\nThe total cost of all the laptops is N{}", hp.tot()+ ibm.tot()+dell.tot()+toba.tot()); 
}

fn display(n:&String,m:u32){
	println!("The total price of the {} laptops is N{}", n,m );
}