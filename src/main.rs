mod utils;
use utils::array::{mod_arr, print_arr};
use utils::file_handling::{read_file, write_file_simple, write_file_with_match};

fn main() {
    // This is for the array module
    let mut my_string_array: [String; 3] =
        ["Hello".to_string(), "World".to_string(), "!".to_string()];
    let mut my_int_array: [i8; 6] = [1, 2, 3, 4, 5, 6];
    let mut my_float_array: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let mut my_i32_array: [i32; 4] = [10, 20, 30, 40];

    print_arr(&my_string_array);
    print_arr(&my_int_array);

    // Call mod_arr on string array
    if let Some(new_string_vec) = mod_arr(&mut my_string_array) {
        println!("New string array created: {:?}", new_string_vec);
    }

    // Call mod_arr on int array - this modifies in place
    println!("Before modification:");
    print_arr(&my_int_array);

    mod_arr(&mut my_int_array);

    println!("After modification (odd indices should be incremented by 1):");
    print_arr(&my_int_array);

    // Test with float array
    println!("\nTesting with f32 array:");
    println!("Before modification:");
    print_arr(&my_float_array);

    mod_arr(&mut my_float_array);

    println!("After modification (odd indices should be incremented by 1.0):");
    print_arr(&my_float_array);

    // Test with i32 array
    println!("\nTesting with i32 array:");
    println!("Before modification:");
    print_arr(&my_i32_array);

    mod_arr(&mut my_i32_array);

    println!("After modification (odd indices should be incremented by 1):");
    print_arr(&my_i32_array);

    println!("\nHello, world!");

    // Example usage of file handling functions
    let content = "Hello from Rust file handling!";

    // Using the simple write function with multiple lines in each arm
    match write_file_simple("test.txt", content) {
        Ok(_) => {
            println!("File written successfully!");
            println!(
                "Logging: Operation completed at {}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            println!("File size: {} bytes", content.len());
        }
        Err(e) => {
            println!("Error writing file: {}", e);
            println!(
                "Logging: Error occurred at {}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            println!("Attempting to create backup...");
            // You could add backup logic here
        }
    }

    // Using the match-based write function with multiple lines
    match write_file_with_match("test2.txt", content) {
        Ok(x) => {
            println!("File written with match successfully! with output = {}", x);
            println!("Additional processing for successful write...");
            println!("Validating file contents...");
            // You could add validation logic here
        }
        Err(e) => {
            println!("Error writing file: {}", e);
            println!("Error type: {:?}", e.kind());
            println!("Attempting recovery...");
            // You could add recovery logic here
        }
    }

    // Reading a file
    match read_file("test.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
