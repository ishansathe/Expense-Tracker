use std::{fs, mem};
use std::mem::take;

pub fn read_item_cost_entries() -> Vec<String>{

    let byte_array : Vec<u8> = fs::read("./ItemCosts.txt").unwrap();
    // I'm not expecting any errors, so I just skipped the error handling by including 'unwrap()' at the end.
    // It removes the 'Ok' wrap and returns th e actual result value.

    // Stores the costs one by one
    let mut cost_entries: Vec<String> = Vec::new();

    // Used to concatenate digits together
    let mut num_string: String = String::from("");

    for byte in byte_array{

        // If a byte value of '10' ('\n' in byte format) is encountered.
        if byte == 10 {
            
            // It doesn't matter to convert back into integer, because it is being stored as string in slint anyway.
            cost_entries.push(mem::take(&mut num_string));
        }
        else {
            // Using direct typecasting to push byte as a character onto the string 
            // The 'push' function expects a character too!
            num_string.push(byte as char);
        }
    }
    
    // println!("Now, the vector contains: {:?}", cost_entries);

    return cost_entries;
}


pub fn read_item_name_entries () -> Vec<String>{

    let byte_array = fs::read("./ItemNames.txt").unwrap();

    let mut name_entries: Vec<String> = Vec::new();

    let mut name_string: String = String::from("");

    for byte in byte_array {
        if byte == 10 {

            /* 
            Now, the recommended option was to use name_string.clone(). 
            But that creates many identical copies on the heap memory. As the size of entries increase,
            More RAM will be eaten up and the process will become slower. Plus, many empty strings stored on heap.

            // name_entries.push(name_string.clone());

            The usage of 'clone()' was necessary because otherwise, the ownership of 'name_string', which is a String
            is given to 'name_entries'. This is because String does not have a Copy trait 
            (that allows you to simply copy stuff.)
            */

            /* 
            To deal with the above issue, the value stored at memory of the 'reference of String' -> '&mut String'
            is then sent as a value to the name_entries vector. The string no longer needs to be cloned. 
            This is done by the 'take' function from std::mem. 
            It takes all the values, leaving an empty string. This removes the need for .clear() function too!
            */
            name_entries.push(take(&mut name_string));

        }
        else {
            name_string.push(byte as char);
        }
    }

    // println!("{:?}", name_entries);

    return name_entries;
}

