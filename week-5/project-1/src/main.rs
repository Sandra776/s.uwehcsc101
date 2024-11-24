use std::collections::HashMap;
use std::io;

fn main() {
    let mut menu = HashMap::new();
    menu.insert("P", 3200); // Pounded Yam/Edinkaiko Soup
    menu.insert("F", 3000); // Fried Rice & Chicken
    menu.insert("A", 2500); // Amala & Ewedu Soup
    menu.insert("E", 2000); // Eba & Egusi Soup
    menu.insert("W", 2500); // White Rice & Stew

    println!("Menu:");
    println!("P = Pounded Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    let mut total_cost = 0;
    loop {
        println!("Enter the food code or type 'done' to submit");
        let mut food_code = String::new();
        io::stdin().read_line(&mut food_code).unwrap();
        let food_code = food_code.trim();

        if food_code == "done" {
            break;
        }

        if let Some(&price) = menu.get(food_code) {
            println!("Enter quantity: ");
            let mut qty = String::new();
            io::stdin().read_line(&mut qty).unwrap();
            let qty: i32 = qty.trim().parse().unwrap_or(0);

            total_cost += price * qty;
        } else {
            println!("Please Enter Valid food code");
        }
    }

 
    if total_cost > 10_000 {
        println!("You qualify for a 5% discount!");
        total_cost = (total_cost as f64 * 0.95) as i32;
    }


    println!("Your total cost is: N{}", total_cost);
}

