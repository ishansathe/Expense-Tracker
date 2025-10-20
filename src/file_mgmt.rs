
use std::fs::{self, OpenOptions};
use std::io::Write;

use serde_json::{from_str, json, to_string_pretty, Value};

use crate::serdes::ItemInfo;

pub fn store_in_file(name: String, cost: String, category: String){

    // Opening a file object with write options to write the json data into.
    // We now always paste a new array into this file.
    let mut write_file_object = OpenOptions::new()
        .create(true)
        .write(true)
        .open("ItemDetails.json").unwrap();
    
    // Storing existing JSON Array in string format, here.
    let read_file = fs::read_to_string("ItemDetails.json").unwrap();

    // Creating a new item to be stored inside the JSON Array.
    let new_item = ItemInfo {
        name: name,
        cost: cost,
        category: category
    };

    let json_string;

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


pub fn read_from_file() -> Vec<Vec<String>> {
    let json_file_string = fs::read_to_string("ItemDetails.json").unwrap();

    let mut item_vector = from_str::<Vec<ItemInfo>>(&json_file_string).unwrap();

    let mut vec_item_name: Vec<String> = Vec::with_capacity(item_vector.len());
    let mut vec_item_cost: Vec<String> = Vec::with_capacity(item_vector.len());

    let mut vec_item_category: Vec<String> = Vec::with_capacity(item_vector.len());
    // let mut vec_item_category: Vec<Category> = Vec::with_capacity(item_vector.len());

    for index in 0 .. item_vector.len() {

        vec_item_name.push(std::mem::take(& mut item_vector[index].name));

        vec_item_cost.push(std::mem::take(& mut item_vector[index].cost));

        vec_item_category.push(std::mem::take(& mut item_vector[index].category));
    }

    let mut name_cost_category_vectors: Vec<Vec<String>> = Vec::with_capacity(3);

    name_cost_category_vectors.push(vec_item_name);

    name_cost_category_vectors.push(vec_item_cost);

    name_cost_category_vectors.push(vec_item_category);

    return name_cost_category_vectors;
}


pub fn calculate_total() -> i32 {
    let item_file = fs::read_to_string("ItemDetails.json").unwrap();

    let item_file_json  = from_str::<Vec<ItemInfo>>(&item_file).unwrap();
    // println!("{:?}", item_file_json);

    let mut total: i32 = 0;

    for i in 0 .. item_file_json.len() {
        total += item_file_json[i].cost
            .parse::<i32>()
            .unwrap();
    }

    return total;
}

pub fn get_budgets( _category: &str ) -> i32 {

    let items_file = fs::read_to_string("ItemDetails.json").unwrap();

    let items_file_json = from_str::<Vec<ItemInfo>>(&items_file).unwrap();

    let mut category_total : i32 = 0;

    for i in 0 .. items_file_json.len() {
        if items_file_json[i].category == _category {
            category_total += items_file_json[i].cost
                .parse::<i32>()
                .unwrap();
        }
    }

    return category_total;
}