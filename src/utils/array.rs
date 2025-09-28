use std::collections::HashMap;

// Enum to handle different return types
#[derive(Debug)]
pub enum ModArrResult<T> {
    ModifiedValues(HashMap<usize, T>),  // For integers/floats
    NewArray(Vec<T>),                   // For strings/&str
    Error(String),                      // For unsupported types or any error occurred in the function
}
#[allow(dead_code)]
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

// Trait to identify supported types
pub trait SupportedType {
    fn is_supported() -> bool;
    fn type_name() -> &'static str;
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

impl SupportedType for i8 {
    fn is_supported() -> bool {
        true
    }
    fn type_name() -> &'static str {
        "i8"
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

impl SupportedType for i32 {
    fn is_supported() -> bool {
        true
    }
    fn type_name() -> &'static str {
        "i32"
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

impl SupportedType for f32 {
    fn is_supported() -> bool {
        true
    }
    fn type_name() -> &'static str {
        "f32"
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

impl SupportedType for f64 {
    fn is_supported() -> bool {
        true
    }
    fn type_name() -> &'static str {
        "f64"
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

impl SupportedType for String {
    fn is_supported() -> bool {
        true
    }
    fn type_name() -> &'static str {
        "String"
    }
}

// Example of unsupported type
impl ModifiableArray for bool {
    fn modify_array(&mut self, _index: usize) {
        // Don't modify bool values
    }
    fn should_return_copy() -> bool {
        false
    }
}

impl SupportedType for bool {
    fn is_supported() -> bool {
        false
    }
    fn type_name() -> &'static str {
        "bool"
    }
}

// Support for &str
impl ModifiableArray for &str {
    fn modify_array(&mut self, _index: usize) {
        // Don't modify &str values
    }
    fn should_return_copy() -> bool {
        true
    }
}

impl SupportedType for &str {
    fn is_supported() -> bool {
        true
    }
    fn type_name() -> &'static str {
        "&str"
    }
}

// New version that returns different types based on array type, with error handling for unexpected cases
pub fn mod_arr<T>(array: &mut [T]) -> ModArrResult<T>
where
    T: ModifiableArray + Clone + 'static + SupportedType,
{
    // Check if the type is supported
    if !T::is_supported() {
        return ModArrResult::Error(format!(
            "Unsupported types of array: {}. Use integers, floats, or string arrays",
            T::type_name()
        ));
    }

    // Simple, direct approach - no need for catch_unwind for these operations
    if T::should_return_copy() {
        // For strings, create and return a new array
        let new_array = array.to_vec();
        ModArrResult::NewArray(new_array)
    } else {
        // For numeric types, modify odd-indexed items in place and track changes
        let mut modified_map = HashMap::new();
        for (index, item) in array.iter_mut().enumerate() {
            item.modify_array(index);

            // Only add to map if the value actually changed (odd indices)
            if index % 2 == 1 {
                modified_map.insert(index, item.clone());
            }
        }
        ModArrResult::ModifiedValues(modified_map)
    }
}

#[allow(dead_code)]
// Example of proper error handling for operations that can actually fail
pub fn mod_arr_with_validation<T>(array: &mut [T]) -> ModArrResult<T>
where
    T: ModifiableArray + Clone + 'static + SupportedType,
{
    // Check if the type is supported
    if !T::is_supported() {
        return ModArrResult::Error(format!(
            "Unsupported types of array: {}. Use integers, floats, or string arrays",
            T::type_name()
        ));
    }

    // Validate array length (example of a check that could fail)
    if array.is_empty() {
        return ModArrResult::Error("Array cannot be empty".to_string());
    }

    if array.len() > 1000 {
        return ModArrResult::Error("Array too large (max 1000 elements)".to_string());
    }

    // Now do the actual work - these operations are infallible
    if T::should_return_copy() {
        let new_array = array.to_vec();
        ModArrResult::NewArray(new_array)
    } else {
        let mut modified_map = HashMap::new();
        for (index, item) in array.iter_mut().enumerate() {
            item.modify_array(index);
            if index % 2 == 1 {
                modified_map.insert(index, item.clone());
            }
        }
        ModArrResult::ModifiedValues(modified_map)
    }
}