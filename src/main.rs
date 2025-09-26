mod funcs;
use funcs::file_handling::{read_file, write_file_with_match, write_file_simple};

fn main() {
    println!("Hello, world!");
    
    // Example usage of file handling functions
    let content = "Hello from Rust file handling!";
    
    // Using the simple write function with multiple lines in each arm
    match write_file_simple("test.txt", content) {
        Ok(_) => {
            println!("File written successfully!");
            println!("Logging: Operation completed at {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
            println!("File size: {} bytes", content.len());
        },
        Err(e) => {
            println!("Error writing file: {}", e);
            println!("Logging: Error occurred at {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
            println!("Attempting to create backup...");
            // You could add backup logic here
        },
    }
    
    // Using the match-based write function with multiple lines
    match write_file_with_match("test2.txt", content) {
        Ok(x) => {
            println!("File written with match successfully! with output = {}", x);
            println!("Additional processing for successful write...");
            println!("Validating file contents...");
            // You could add validation logic here
        },
        Err(e) => {
            println!("Error writing file: {}", e);
            println!("Error type: {:?}", e.kind());
            println!("Attempting recovery...");
            // You could add recovery logic here
        },
    }
    
    // Reading a file
    match read_file("test.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
