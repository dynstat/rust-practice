#![allow(unused)]
use std::fmt::{self, Display};

// Why Dog<T> doesn't require a lifetime parameter (e.g., <'a>):
// 1. T is a generic type parameter (wildcard). It can stand in for any type.
// 2. A reference type like `&'a String` is a type.
// 3. When you instantiate `Dog { name: &s, .. }`, the compiler infers `T` to be `&'a String`.
//    The lifetime `'a` is carried within the type substituted for `T`.
//
// Contrast: If we did not use generics and wrote `struct Dog { name: &String }`, 
// we would be using the `&` symbol directly and must declare a lifetime: `struct Dog<'a> { name: &'a String }`.
struct Dog<T> {
    name: T,
    legs: i32,
}

struct Bird<T> {
    name: T,
    legs: i32,
    can_fly: bool,
}

trait Description {
    fn get_desc(&self) -> String {
        format!("Default get_desc() called !!")
    }
}

impl<T> Description for Dog<T>
where
    T: Display,
{
    fn get_desc(&self) -> String {
        print!("The dog is {} and it has {} legs.\n", self.name, self.legs);
        let s = format!("The dog is {} and it has {} legs.\n", self.name, self.legs);
        s
    }
}
impl<T> Description for Bird<T>
where
    T: Display,
{
    // fn get_desc(&self) -> String {
    //     print!("The Bird is {} and it has {} legs.\n", self.name, self.legs);
    //     format!("The Bird is {} and it has {} legs.\n", self.name, self.legs)
    // }
}
fn print_desc<T>(animal: &T)
where
    T: Description,
{
    // ERROR : Rust only knows that T implements the trait Desc.
    // It has no knowledge of the struct’s fields (legs, name, etc.),
    // because traits don’t expose fields — they only expose methods.
    // println!("{}", animal.legs); // <======

    println!("{}", animal.get_desc());
}

fn main() {
    let s = String::from("tommy");

    // Here, T is inferred as `&'a String` (a reference type with s's lifetime).
    // The struct becomes Dog<&'a String>, which is why no explicit lifetime on Dog was needed.
    let dog1 = Dog { name: &s, legs: 4 };

    let dog2 = Dog {
        name: "kutta".to_string(),
        legs: 4,
    };
    // dog1.get_desc(); // Correct
    // dog1.get_desc(); // Correct
    let bird1 = Bird {
        name: "chidiya",
        legs: 2,
        can_fly: true,
    };
    // let x = bird1.get_desc(); // Correct
    // println!("{x}");

    print_desc(&dog1);
    print_desc(&dog2);
    print_desc(&bird1);
}
