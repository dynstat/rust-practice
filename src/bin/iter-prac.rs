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
    // as_deref() and as_ref(), both are used to get the immutable reference to a field of the struct. So that the struct can be used or accessed later
    let temp = l2.next.as_deref().map(|node| node.data);
    println!("{:#?}", l2);
    let mut l1 = LList {
        data: 100,
        next: Some(Box::new(l2)),
    };
    // let l2_data = l1.next.unwrap(); // this is the partial move of field from the struct. Better to use the as_deref()

    let l2_data = l1.next.as_deref_mut().map(|n| {
        n.data *= 2;
        n.data
    });

    // println!("{:#?} {:#?}", l2.data, l1); // INVALID as l2 was moved inside of the l1 box.
}
