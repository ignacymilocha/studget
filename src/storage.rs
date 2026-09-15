use crate::templates::Groceries;
use std::fs;

const FILE_PATH: &str = "studget_data.json";

pub fn save_groceries(list: &Vec<Groceries>) {
    let json = serde_json::to_string_pretty(list).unwrap();
    fs::write(FILE_PATH, json).expect("Failed to save data");
}

pub fn load_groceries() -> Vec<Groceries> {
    match fs::read_to_string(FILE_PATH) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(), // file doesn't exist yet — first run
    }
}