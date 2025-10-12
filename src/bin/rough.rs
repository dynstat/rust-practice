use std::any::type_name;

fn print_type_of<T>(your_var: &T)
where
    T: std::fmt::Debug,
{
    println!("Type of {:#?}: {}", your_var, type_name::<T>());
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
    print_type_of(&x);
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
    print_type_of(&y);
}

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);
}

