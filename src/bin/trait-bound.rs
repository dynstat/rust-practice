#![allow(unused)]
use std::{alloc, fmt::Display};

struct Dog<T> {
    name: T,
    legs: i32,
}

struct Bird<T> {
    name: T,
    legs: i32,
    can_fly: bool,
}

trait Desc {
    fn get_desc(&self) -> String {
        format!("Default get_desc() called !!")
    }
}

impl<T> Desc for Dog<T>
where
    T: Display,
{
    fn get_desc(&self) -> String {
        print!("The dog is {} and it has {} legs.\n", self.name, self.legs);
        format!("The dog is {} and it has {} legs.\n", self.name, self.legs)
    }
}
impl<T> Desc for Bird<T>
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
    T: Desc,
{
    // ERROR : Rust only knows that T implements the trait Desc.
    // It has no knowledge of the struct’s fields (legs, name, etc.),
    // because traits don’t expose fields — they only expose methods.
    // println!("{}", animal.legs); // <======

    println!("{}", animal.get_desc());
}

fn main() {
    let dog1 = Dog {
        name: "kutta",
        legs: 4,
    };
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
