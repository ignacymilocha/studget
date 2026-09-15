use crate::templates::Groceries;

pub fn add_grocery(list: &mut Vec<Groceries>, next_id: &mut u32, name: String, price: f64) {
    list.push(Groceries {
        id: *next_id,
        name: name.trim().to_string(),
        price: price,
    });
    println!("{} ({}$) has been added to the list!", name.trim(), price);
    *next_id += 1;
}

pub fn list_groceries(list: &Vec<Groceries>) {
    for item in list {
        println!("{} | {} | {}$", item.id, item.name, item.price);
    }
}

pub fn set_budget(budget: &mut Option<f32>, spendings: &mut f32, amount: f32) {
    *budget = Some(amount);
    *spendings = 0.0;
    println!("Budget set to {}$", amount);
}

pub fn show_budget(budget: &Option<f32>, spendings: f32) {
    match budget {
        Some(amount) => {
            println!("Budget: {}$", amount);
            println!("Spendings: {}$", spendings);
            println!("Remaining: {}$", amount - spendings);
        }
        None => {
            println!("No budget set.");
        }
    }
}

pub fn add_budget(budget: &mut Option<f32>, amount: f32) {
    match budget {
        Some(current_budget) => {
            *current_budget += amount;
            println!("Budget increased by {}$. New budget: {}$", amount, current_budget);
        }
        None => {
            *budget = Some(amount);
            println!("Budget set to {}$", amount);
        }
    }
}