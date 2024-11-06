// generic enum, type can be anything, already exist in rust
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;

fn main() {
    let res = fs::read_to_string("example.txt");

    match res {
        Ok(content) => { 
            println!("File content: {}", content);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
