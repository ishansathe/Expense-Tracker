
use std::fs::{OpenOptions};
use std::io::Write;


use serde_json::from_str;

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
