use std::fmt::Debug;

pub enum MyTypes {
    STR1(&'static str),
    INT32(i32),
    FT64(f64),
}

// use this for passing data to the function when we know the type of the data and we are not using generics for the function.
pub fn test_types(some_type: MyTypes) {
    match some_type {
        MyTypes::STR1(x) => println!("this is {:?} of type string slice", x),
        MyTypes::INT32(x) => println!("this is {:?} of type i32", x),
        MyTypes::FT64(x) => println!("this is {:?} of type f64", x),
    }
}

use std::any::{Any, TypeId};

#[allow(dead_code)]
pub fn test_types_match_typeid(value: &dyn Any) {
    match value.type_id() {
        id if id == TypeId::of::<i32>() => {
            println!("hehe i32: {}", value.downcast_ref::<i32>().unwrap())
        }
        id if id == TypeId::of::<f64>() => {
            println!("hehe f64: {}", value.downcast_ref::<f64>().unwrap())
        }
        id if id == TypeId::of::<String>() => {
            println!("hehe String: {}", value.downcast_ref::<String>().unwrap())
        }
        id if id == TypeId::of::<&str>() => {
            println!("hehe &str: {}", value.downcast_ref::<&str>().unwrap())
        }
        _ => println!("Unsupported type"),
    }
}

#[allow(dead_code)]
pub fn test_types_generics<T>(some_type: T)
where
    T: Debug,
{
    println!("generic Debug value: {:?}", some_type);
}

// ------------------------------------------------------------
// Generic + trait approach: call with i32, f64, String, &str, etc.
// ------------------------------------------------------------

#[allow(dead_code)]
pub trait TypeAction {
    fn handle(self);
}

impl TypeAction for i32 {
    fn handle(self) {
        println!("i32: {}", self);
    }
}

impl TypeAction for f64 {
    fn handle(self) {
        println!("f64: {}", self);
    }
}

impl TypeAction for String {
    fn handle(self) {
        println!("String: {}", self);
    }
}

impl TypeAction for &str {
    fn handle(self) {
        println!("&str: {}", self);
    }
}

// Function that accepts any type implementing TypeAction
#[allow(dead_code)]
pub fn test_types_trait<T: TypeAction>(value: T) {
    value.handle();
}
