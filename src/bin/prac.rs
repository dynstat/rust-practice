#![allow(unused)]
// Common standard library imports
use std::collections::{HashMap, HashSet}; // For data structures
use std::fs::File; // For file operations
use std::io::{self, Read, Write}; // For terminal I/O
use std::path::Path; // For safe path handling

/// A simple custom error type for your application
type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    println!("Write something... \n");
    let input_size = io::stdin().read_line(&mut input).unwrap_or(0);

    // let s = if let Ok(size) = input_size_res {
    //     size
    // } else {
    //     0
    // };

    println!(
        "user entered {} and the size is {} bytes.",
        input.trim(),
        input.trim().len()
    );

    let status = read_file(input.trim())?;
    if status == true {
        print!("\nFile found and read its content successfully...\n");
    };
    print!("{status:?}");
    Ok(())
}

fn read_file(trimmed_path: &str) -> Result<bool, io::Error> {
    let mut file_res = File::open(trimmed_path)?;

    let mut content: String = String::new();
    file_res.read_to_string(&mut content)?;

    print!("Content: {}", content);

    Ok(true)
}
