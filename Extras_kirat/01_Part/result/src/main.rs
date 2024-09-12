// result:

// write a function that reads the contents of a file:

use std::fs;
fn main() {
    let result = fs::read_to_string("a.txt");  // file may or may not be exist
    match result {
        Ok(data) => println!("Read successfully = {:?}", data),
        Err(err) => println!("error occur while readig: {:?}", err),
    }
}
