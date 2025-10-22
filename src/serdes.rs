use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemInfo{
    pub name: String,
    pub cost: String,
    pub category: String,
}
