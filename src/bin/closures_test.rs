fn main() {
    let v: Vec<i32> = vec![1, 2, 3];

    // move forces ownership into closure
    let c = move || {
        println!("{:?}", v);
    };

    // What traits does `c` implement?

    // It implements:
    // Fn (because it doesn't mutate or consume)
    // FnMut (supertrait)
    // FnOnce (supertrait)

    c(); // OK
    c(); // ALSO OK because closure is not consuming its captured state
}
