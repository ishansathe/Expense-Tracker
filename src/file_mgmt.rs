
use std::fmt::format;
use std::fs::{self, read, OpenOptions};
use std::io::Write;


use serde::de::value;
use serde_json::{from_str, json, to_string_pretty, Value};

use crate::serdes::ItemInfo;

// This function is used to write Item name and cost to a file.
pub fn write_to_files (name: String, cost: String) {

    let mut item_name_file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true) //create if it doesn't exist
        .open("ItemNames.txt").unwrap();

    let mut item_cost_file = OpenOptions::new()
        .read(true)
        .append(true) //appends instead of overwriting.
        .create(true)
        .open("ItemCosts.txt").unwrap();

    let _name = name.clone() + "\n";
    let _cost = cost.clone() + "\n";

    item_name_file.write(_name.as_bytes())
        .expect("Could not write to the naming file.");

    item_cost_file.write( _cost.as_bytes())
        .expect("Could not write to the costing file");
    

}

#[allow(dead_code)]
pub fn write_json_to_files (json_item_string: String){

    let mut item_details_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("ItemDetails.json").unwrap();

    item_details_file.write(json_item_string.as_bytes())
        .expect("Could not write to ItemDetails.json");
}


#[allow(dead_code)]
pub fn read_json_from_file () -> ItemInfo{

    let item_details_file = std::fs::read_to_string("ItemDetails.json").unwrap();

    let item_details_json_struct = from_str::<ItemInfo>(&item_details_file).unwrap();

    return item_details_json_struct;
}


pub fn store_in_file(name: String, cost: String, category: String){

    // Opening a file object with write options to write the json data into.
    // We now always paste a new array into this file.
    let mut write_file_object = OpenOptions::new()
        .create(true)
        .write(true)
        .open("ItemDetails.json").unwrap();
    
    // Storing existing JSON Array in string format, here.
    let read_file = fs::read_to_string("ItemDetails.json").unwrap();

    println!("{}", read_file);

    // Creating a new item to be stored inside the JSON Array.
    let new_item = ItemInfo {
        name: name,
        cost: cost,
        category: category
    };

    let mut json_string;

    // In case the file has never been created before.
    if read_file != "" {
        // Now, I have to convert it into a JSON format.
        let mut json_array: Value = serde_json::from_str(&read_file).unwrap();

        if let Some(json_array) = json_array.as_array_mut() {
            
            let new_json_item = json!(new_item);
            json_array.push(new_json_item);
        }

        // In Rust, or in core programming languages, JSON can't be stored as is. 
        // It must be properly converted into String format of json.
        json_string = to_string_pretty(&json_array).unwrap();
    }

    // Now, arrays will be stored even for newer values.
    else {
        let new_json_item: Value = json!(new_item);


        // In Rust, or in core programming languages, JSON can't be stored as is. 
        // It must be properly converted into String format of json.
        let unready_json_string = to_string_pretty(&new_json_item).unwrap();

        json_string = format!("[{}]", unready_json_string);
    }
    

    // Now, to write onto the file:
    // The string must also be converted into bytes.
    // For by default, only bytes are stored by this function.
    write_file_object
        .write(json_string.as_bytes())
        .expect("JSON should have been written but failed");

}