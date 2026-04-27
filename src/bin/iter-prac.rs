#![allow(unused)]
// Common standard library imports
use std::collections::{HashMap, HashSet}; // For data structures
use std::fs::File; // For file operations
use std::io::{self, Write}; // For terminal I/O
use std::iter::Iterator;
use std::path::Path; // For safe path handling
/// A simple custom error type for your application
type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct LList {
    data: i32,
    next: Option<Box<LList>>,
}

fn main() {
    let l2 = LList {
        data: 99,
        next: None,
    };
    println!("{:?} - {:?}", l2.data, l2.next);
    let temp = l2
        .next
        .unwrap_or(Box::new(LList {
            data: 99,
            next: None,
        }))
        .data;
    println!("{:?}", temp);
    // let l1 = LList {
    //     data: 100,
    //     next: Some(Box::new(l2)),
    // };
    // let l2_data = l1.next.unwrap();
    // println!("{:?}", l1.data);
}
