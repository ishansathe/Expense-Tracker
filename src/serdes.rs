use serde::{Serialize,Deserialize};
use serde_json::{from_str, Value};
// use serde_json::to_string_pretty;

use std::fs::{ OpenOptions };
// use std::fs::read_to_string;
use std::io::{ Write};

// use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemInfo{
    pub name: String,
    pub cost: String,
    pub category: String,
}

#[allow(dead_code)]
fn main() {
    serialize_struct_into_json();
    deserialize_json_into_struct();
}

#[allow(dead_code)]
pub fn serialize_struct_into_json(){
    let item1 = ItemInfo{
        name: "Pen".to_string(),
        cost: "20".to_string(),
        category: "Work".to_string(),
    };

    let item2 = ItemInfo{
        name: "Burger".to_string(),
        cost: "120".to_string(),
        category: "Food".to_string(),
    };

    // let vec_items: Vec<ItemInfo> = vec![item1, item2];

    // let item1_json = to_string_pretty(&vec_items);

    // if item1_json.is_ok() {
    //     println!("{}", item1_json.ok().unwrap());
    // } 
    // else{
    //     println!("{}", item1_json.err().unwrap());
    // }

    let item_details_string = std::fs::read_to_string("ItemDetails.json").unwrap();

    
    let mut root: Value = from_str(&item_details_string).unwrap();

    if let Some(items_array) = root.as_array_mut() {
        // items_array.push(serde_json::json!(item1_json).into());

        // let ok: Value = serde_json::json!(item1_json.ok().unwrap());

        let json_item1: Value = serde_json::json!(item1);
        let json_item2: Value = serde_json::json!(item2);
        // Converting the JSON string into proper json.

        items_array.push(json_item1);
        items_array.push(json_item2);
    }

    let updated_string = serde_json::to_string_pretty(&root).unwrap();

    let mut mut_file = OpenOptions::new()
    .create(true)
    .write(true)
    .open("ItemDetails.json").unwrap();

    let _ok = mut_file.write(updated_string.as_bytes()).unwrap();


}

#[allow(dead_code)]
pub fn deserialize_json_into_struct() {

    let raw_string = r#"
    [{
        "name": "Book",
        "cost": "45",
        "category": "Work"
    }]
    "#;

    let item1 = from_str::<Vec<ItemInfo>>(raw_string);

    if item1.is_ok(){
        println!("{:?}", item1.ok().unwrap());
    }
    else {
        println!("{}", item1.err().unwrap());
    }
}
