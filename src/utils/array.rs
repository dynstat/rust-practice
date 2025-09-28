pub fn print_arr<T>(array: &[T])
where
    T: std::fmt::Debug,
{
    let mut count = 0;
    for item in array {
        println!("count number {:?} : {:?}", count, item);
        count += 1;
    }
}

// Trait for types that can be modified
pub trait ModifiableArray {
    fn modify_array(&mut self, index: usize);
    fn should_return_copy() -> bool;
}

impl ModifiableArray for i8 {
    fn modify_array(&mut self, index: usize) {
        if index % 2 == 1 {
            *self += 1;
        }
    }
    fn should_return_copy() -> bool {
        false
    }
}

impl ModifiableArray for i32 {
    fn modify_array(&mut self, index: usize) {
        if index % 2 == 1 {
            *self += 1;
        }
    }
    fn should_return_copy() -> bool {
        false
    }
}

impl ModifiableArray for f32 {
    fn modify_array(&mut self, index: usize) {
        if index % 2 == 1 {
            *self += 1.0;
        }
    }
    fn should_return_copy() -> bool {
        false
    }
}

impl ModifiableArray for f64 {
    fn modify_array(&mut self, index: usize) {
        if index % 2 == 1 {
            *self += 1.0;
        }
    }
    fn should_return_copy() -> bool {
        false
    }
}

impl ModifiableArray for String {
    fn modify_array(&mut self, _index: usize) {
        // Don't modify strings
    }
    fn should_return_copy() -> bool {
        true
    }
}

pub fn mod_arr<T>(array: &mut [T]) -> Option<Vec<T>>
where
    T: ModifiableArray + Clone + 'static,
{
    if T::should_return_copy() {
        // For strings, create and return a new vector
        return Some(array.to_vec());
    }

    // For numeric types, modify odd-indexed items in place
    for (index, item) in array.iter_mut().enumerate() {
        item.modify_array(index);
    }

    None // No return for numeric types
}

// Specialized version for &str since it can't be modified in place
#[allow(dead_code)]
pub fn mod_arr_str(array: &[&str]) -> Vec<String> {
    array.iter().map(|s| s.to_string()).collect()
}

// #[allow(dead_code)]
// pub fn process_array_items<T: std::fmt::Display + std::fmt::Debug>(array: &[T])
// where
//     T: std::any::Any,
// {
//     use std::any::TypeId;

//     for (index, item) in array.iter().enumerate() {
//         if TypeId::of::<T>() == TypeId::of::<String>() {
//             println!("Item {} is a String: {}", index, item);
//         } else if TypeId::of::<T>() == TypeId::of::<i32>() {
//             println!("Item {} is an i32: {:?}", index, item);
//         } else if TypeId::of::<T>() == TypeId::of::<f64>() {
//             println!("Item {} is an f64: {:?}", index, item);
//         } else if TypeId::of::<T>() == TypeId::of::<bool>() {
//             println!("Item {} is a bool: {:?}", index, item);
//         } else {
//             println!("Item {} is of unknown type: {}", index, item);
//         }
//     }
// }
