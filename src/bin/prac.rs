#![allow(unused)]
// Common standard library imports
use std::collections::{HashMap, HashSet}; // For data structures
use std::fs::File; // For file operations
use std::io::{self, Write}; // For terminal I/O
use std::path::Path; // For safe path handling

/// A simple custom error type for your application
type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() {
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
        input.trim().len(),
        input.trim()
    );
}
