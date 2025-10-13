
use std::fs::{OpenOptions};
use std::io::Write;


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

