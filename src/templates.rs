use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Groceries {
    pub id: u32,
    pub name: String,
    pub price: f64,
}