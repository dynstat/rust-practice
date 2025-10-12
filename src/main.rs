mod utils;
use utils::array::mod_arr;
use utils::checktypes::{MyTypes, test_types};
use utils::file_handling::{read_file, write_file_simple, write_file_with_match};

#[allow(dead_code)]
fn test_arrays() {
    // This is for the array module
    let mut my_string_array: [String; 3] =
        ["Hello".to_string(), "World".to_string(), "!".to_string()];
    let mut my_int_array: [i8; 6] = [1, 2, 3, 4, 5, 6];
    let mut my_float_array: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let mut my_i32_array: [i32; 4] = [10, 20, 30, 40];
    let mut my_str_array: [&str; 3] = ["foo", "bar", "baz"];
    println!("Original &str array:");
    // print_arr(&my_str_array);

    match mod_arr(&mut my_str_array) {
        utils::array::ModArrResult::NewArray(new_array) => {
            println!("The returned value is {:?}", new_array);

            for (index, value) in new_array.iter().enumerate() {
                println!("str Index {}: {}", index, value);
            }
        }
        // utils::array::ModArrResult::Error(e) => println!("Error: {}", e),
        _ => {}
    }

    // Call mod_arr on string array
    match mod_arr(&mut my_string_array) {
        utils::array::ModArrResult::NewArray(new_array) => {
            println!("String array processed successfully! New array created:");
            for (index, value) in new_array.iter().enumerate() {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::ModifiedValues(modified_map) => {
            println!("String array modified values:");
            for (index, value) in modified_map {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::Error(e) => println!("Error: {}", e),
    }

    // Call mod_arr on int array - this modifies in place
    println!("Before modification:");
    // print_arr(&my_int_array);

    match mod_arr(&mut my_int_array) {
        utils::array::ModArrResult::ModifiedValues(modified_map) => {
            println!("Integer array modified successfully!");
            println!("Modified values: {:?}", modified_map);
            // for (index, value) in modified_map {
            //     println!("Index {}: {}", index, value);
            // }
        }
        utils::array::ModArrResult::NewArray(new_array) => {
            println!("Integer array new array created:");
            for (index, value) in new_array.iter().enumerate() {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::Error(e) => println!("Error: {}", e),
    }

    println!("After modification (odd indices should be incremented by 1):");
    // print_arr(&my_int_array);

    // Test with float array
    println!("\nTesting with f32 array:");
    println!("Before modification:");
    // print_arr(&my_float_array);

    match mod_arr(&mut my_float_array) {
        utils::array::ModArrResult::ModifiedValues(modified_map) => {
            println!("Float array modified successfully!");
            println!("Modified values: {:?}", modified_map);
            // for (index, value) in modified_map {
            //     println!("Index {}: {}", index, value);
            // }
        }
        utils::array::ModArrResult::NewArray(new_array) => {
            println!("Float array new array created:");
            for (index, value) in new_array.iter().enumerate() {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::Error(e) => println!("Error: {}", e),
    }

    println!("After modification (odd indices should be incremented by 1.0):");
    // print_arr(&my_float_array);

    // Test with i32 array
    println!("\nTesting with i32 array:");
    println!("Before modification:");
    // print_arr(&my_i32_array);

    match mod_arr(&mut my_i32_array) {
        utils::array::ModArrResult::ModifiedValues(modified_map) => {
            println!("i32 array modified successfully!");
            println!("Modified values:");
            for (index, value) in modified_map {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::NewArray(new_array) => {
            println!("i32 array new array created:");
            for (index, value) in new_array.iter().enumerate() {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::Error(e) => println!("Error: {}", e),
    }

    println!("After modification (odd indices should be incremented by 1):");
    // print_arr(&my_i32_array);
}

#[allow(dead_code)]
fn test_file_handling() {
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

    // Test with unsupported type (bool array)
    println!("\nTesting with unsupported type (bool array):");
    let mut bool_array: [bool; 3] = [true, false, true];
    match mod_arr(&mut bool_array) {
        utils::array::ModArrResult::ModifiedValues(modified_map) => {
            println!("Bool array modified successfully!");
            for (index, value) in modified_map {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::NewArray(new_array) => {
            println!("Bool array new array created:");
            for (index, value) in new_array.iter().enumerate() {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::Error(e) => println!("Error: {}", e),
    }

    // Test with &str array
    println!("\nTesting with &str array:");
    let mut str_array: [&str; 3] = ["Hello", "World", "Rust"];
    match mod_arr(&mut str_array) {
        utils::array::ModArrResult::NewArray(new_array) => {
            println!("&str array processed successfully! New array created:");
            for (index, value) in new_array.iter().enumerate() {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::ModifiedValues(modified_map) => {
            println!("&str array modified values:");
            for (index, value) in modified_map {
                println!("Index {}: {}", index, value);
            }
        }
        utils::array::ModArrResult::Error(e) => println!("Error: {}", e),
    }
}
fn main() {
    // test_arrays();
    // test_file_handling();
    // test_types_match_typeid(&"Hello....");

    test_types(MyTypes::STR1("Hello...."));
    test_types(MyTypes::INT32(99));
    test_types(MyTypes::FT64(99.99));
}
