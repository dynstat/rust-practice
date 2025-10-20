use core::str;
use std::fs;
use std::io::{Read, Write};
// Method 1: Using fs::write (simplest approach)
pub fn write_file_simple(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)?; // ? operator handles the Result
    Ok(())
}

// Method 2: Using match (explicit error handling)
pub fn write_file_with_match(path: &str, content: &str) -> Result<i8, std::io::Error> {
    match fs::write(path, content) {
        Ok(_) => {
            // Multiple lines in success case
            println!("Successfully wrote {} bytes to {}", content.len(), path);
            println!("File operation completed successfully");
            // You could add more processing here
            Ok(0) // Return value at the end
        }
        Err(e) => {
            // Multiple lines in error case
            println!("Failed to write to file: {}", path);
            println!("Error details: {}", e);
            println!("Error kind: {:?}", e.kind());
            // You could add error logging, cleanup, etc. here
            Err(e) // Return the error at the end
        }
    }
}

// Method 3: Using File::create and write_all (more control)
#[allow(dead_code)]
pub fn write_file_detailed(path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path)?; // ? handles the Result<File, Error>
    file.write_all(content.as_bytes())?; // ? handles the Result<(), Error>
    Ok(())
}

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
