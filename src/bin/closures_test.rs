use std::vec;

fn main() {
    let v: Vec<i32> = vec![1, 2, 3];

    // move forces ownership into closure
    let c = || {
        println!("{:?}", v);
        drop(v);
    };

    // print!("{:?}", v); can not be used here as the closure has taken ownership of v.
    c(); // OK
    // c(); // NOT VALID as fnOnce.
}
