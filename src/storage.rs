use crate::templates::Groceries;
use std::fs;

const GROCERY_FILE_PATH: &str = "grocery_data.json";
const BUDGET_FILE_PATH: &str = "budget_data.json";

pub fn save_groceries(list: &Vec<Groceries>) {
    let json = serde_json::to_string_pretty(list).unwrap();
    fs::write(GROCERY_FILE_PATH, json).expect("Failed to save data");
}

pub fn load_groceries() -> Vec<Groceries> {
    match fs::read_to_string(GROCERY_FILE_PATH) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(), // file doesn't exist yet — first run
    }
}

pub fn save_budget(budget: &Option<f32>, spendings: f32) {
    let budget_data = (budget, spendings);
    let json = serde_json::to_string_pretty(&budget_data).unwrap();
    fs::write(BUDGET_FILE_PATH, json).expect("Failed to save budget data");
}

pub fn load_budget() -> (Option<f32>, f32) {
    match fs::read_to_string(BUDGET_FILE_PATH) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| (None, 0.0)),
        Err(_) => (None, 0.0), // file doesn't exist yet — first run
    }
}