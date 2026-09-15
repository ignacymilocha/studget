use std::io::{self, Write};
use std::env;

mod commands;
mod templates;
mod storage;

use templates::Groceries;
use storage::{save_groceries, load_groceries};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: studget <command> [args]");
        return;
    }

    let command = args[1].as_str();

    let mut list: Vec<Groceries> = load_groceries();
    
    let mut next_id: u32 = 0;
    
    let mut budget: Option<f32> = None;
    let mut spendings: f32 = 0.0;
    
    match command {
        "add" | "a" => {
            commands::add_grocery(&mut list, &mut next_id, args[2].clone(), args[3].parse::<f64>().expect("not a number"));
        }
        "list" | "l" => {
            commands::list_groceries(&list);
        }
        "budget" | "b" => {
            match args[2].as_str() {
                "add" | "a" => {
                    if args.len() < 4 {
                        print!("Enter budget: ");
                        io::stdout().flush().unwrap();
                        
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("error");
                        
                        let amount: f32 = input.trim().parse().expect("not a number");
                        commands::set_budget(&mut budget, &mut spendings, amount);
                    } else {
                        let amount: f32 = args[3].parse().expect("not a number");
                        commands::add_budget(&mut budget, amount);
                    }
                }
                _ => {
                    commands::show_budget(&budget, spendings);
                }
            }

            if budget == None {
                
                
                /*print!("You haven't set upt your budget yet! Do you want to? (y/n) ");
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
                }*/
            } else {
                commands::show_budget(&budget, spendings);
            }
        }
        _ => {
           println!("error");
        }
    }

    save_groceries(&list); 
}