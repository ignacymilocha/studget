use std::io::{self, Write};

struct Groceries {
    id: u32,
    name: String,
    price: f32,
}

fn main() {
    let mut list: Vec<Groceries> = Vec::new();
    
    let mut next_id: u32 = 0;
    
    let mut budget: Option<f32> = None;
    
    loop {
        println!("Hello, what do you want to do?");
        print!("(a)dd, (d)elete, (b)udget, (q)uit -> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");
        print!("{}", input);
        
        match input.trim() {
            "a" => {
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("error"); // collects name
                println!("{}", name.trim());
                
                print!("How much?: ");
                io::stdout().flush().unwrap();
                
                let mut price_string = String::new(); 
                io::stdin().read_line(&mut price_string).expect("error");
                let price: f32 = price_string.trim().parse().expect("not a number");
                println!("{}", price);
                
                list.push(Groceries {
                    id: next_id,
                    name: name.trim().to_string(),
                    price: price,
                }); // adds the grocery
                
                println!("{} ({}$) has been added to the list!", name.trim(), price);
                next_id += 1;
            }
            "l" => {
                for item in &list {
                    println!("{} | {} | {}$", item.id, item.name, item.price);
                }
            }
            "b" => {
                if budget == None {
                    print!("You haven't set upt your budget yet! Do you want to? (y/n) ");
                    io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("error");
                    println!("{}", input.trim());
                    
                    match input.trim() {
                        "y" => {
                            print!("Okay! What's your budget? ");
                            io::stdout().flush().unwrap();
                            
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("error");
                            
                            budget = Some(input.trim().parse::<f32>().expect("error"));
                        }
                        "n" => {}
                        _ => {println!("error")}
                    }
                }
            }
            "q" => {
                break;
            }
            _ => {
               println!("error");
            }
        }
    }
}