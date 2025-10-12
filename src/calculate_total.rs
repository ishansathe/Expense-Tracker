use std::fs;

pub fn get_total() -> i32{

    // Directly unwrapping the read operation because the file is expected to be present.
    let byte_array = fs::read("./ItemCosts.txt").unwrap();

    let mut total: i32 = 0;

    let mut num_string = String::new();

    for byte in byte_array {
        if byte == 10 {
            let number: i32 = num_string.parse().unwrap() ;
            total += number;
            num_string.clear();
        } else {
            num_string.push(byte as char);
        }
    }
    // println!("Total is: {}", total);
    return total;
}

// fn main() {
//     get_total();
// }