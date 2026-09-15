use std::io::{self, Write};
use std::env;

mod commands;
mod templates;
mod storage;

use templates::Groceries;
use storage::{save_groceries, load_groceries, save_budget, load_budget};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: studget <command> [args]");
        return;
    }

    let command = args[1].as_str();

    let mut list: Vec<Groceries> = load_groceries();
    
    let mut next_id: u32 = list.iter().map(|g| g.id).max().unwrap_or(0) + 1;
    
    let mut budget = load_budget().0;
    let mut spendings: f32 = load_budget().1;
    
    match command {
        "add" | "a" => {
            commands::add_grocery(&mut list, &mut next_id, args[2].clone(), args[3].parse::<f64>().expect("not a number"));
            commands::update_budget(&mut budget, -args[3].parse::<f32>().expect("not a number"));
        }
        "remove" | "r" => {
            let id: u32 = args[2].parse().expect("not a number");
            commands::update_budget(&mut budget, list.iter().find(|g| g.id == id).map_or(0.0, |g| g.price as f32));
            commands::remove_grocery(&mut list, id);

            for item in &mut list {
                if item.id < id {
                    continue;
                } else {
                    item.id -= 1;
                }
            }
        }
        "list" | "l" => {
            commands::list_groceries(&list);
        }
        "budget" | "b" => {
            if args.len() < 3 {
                commands::show_budget(&budget, spendings);
                return;
            } else {
                match args[2].as_str() {
                    "set" | "s" => {
                        let amount: f32 = args[3].parse().expect("not a number");
                        commands::set_budget(&mut budget, &mut spendings, amount);
                    }
                    "add" | "a" => {
                        let amount: f32 = args[3].parse().expect("not a number");
                        commands::update_budget(&mut budget, amount);
                    }
                    "remove" | "r" => {
                        let amount: f32 = args[3].parse().expect("not a number");
                        commands::update_budget(&mut budget, -amount);
                    }
                    "show" | "sh" => {
                        commands::show_budget(&budget, spendings);
                    }
                    _ => {
                        println!("Invalid budget command. Use 'set', 'show', or 'add'.");
                    }
                }
            }
        }
        _ => {
           println!("error");
        }
    }

    save_groceries(&list); 
    save_budget(&budget, spendings);
}