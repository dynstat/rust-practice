/// ADVANCED LIFETIMES WITH CLOSURES
/// This file covers:
/// 1. Returning closures with multiple input lifetimes
/// 2. Capturing variables and returning references
/// 3. Using Trait Objects (Box<dyn Fn>) with lifetimes

// ---------------------------------------------------------
// CASE 1: Returning a closure that takes two references and returns one
// ---------------------------------------------------------
// We use 'for<'a>' which is a "Higher-Rank Trait Bound" (HRTB).
// It means: "This closure works for ANY lifetime 'a that is provided at call time."
fn get_longer_picker() -> impl for<'a> Fn(&'a str, &'a str) -> &'a str {
    |s1, s2| if s1.len() > s2.len() { s1 } else { s2 }
}

// ---------------------------------------------------------
// CASE 2: Capturing a variable and returning a reference to it
// ---------------------------------------------------------
// The closure "carries" the lifetime 'a of the captured data.
fn create_prefix_checker<'a>(prefix: &'a str) -> impl Fn(&str) -> bool + 'a {
    // We capture 'prefix' which has lifetime 'a
    move |input| input.starts_with(prefix)
}

// ---------------------------------------------------------
// CASE 3: Trait Objects (Box<dyn Fn>) with Lifetimes
// ---------------------------------------------------------
// When using 'dyn', Rust needs to know how long the Box itself is valid.
// '+ 'a' tells Rust the Box contains data valid for lifetime 'a.
fn get_boxed_logger<'a>(tag: &'a str) -> Box<dyn Fn(&str) + 'a> {
    Box::new(move |message| {
        println!("[{}] {}", tag, message);
    })
}

// ---------------------------------------------------------
// CASE 4: Returning a reference to captured data (Advanced)
// ---------------------------------------------------------
fn create_secret_revealer<'a>(secret: &'a str) -> Box<dyn Fn() -> &'a str + 'a> {
    Box::new(move || {
        // This returns a reference to data captured from outside
        secret
    })
}

fn main() {
    // Test Case 1
    let picker = get_longer_picker();
    let result = picker("apple", "banana");
    println!("Longer string: {}", result);

    // Test Case 2 & 3
    let string1 = "ACTIVE";
    let starting_char = "A";
    let prefix_checker = create_prefix_checker(starting_char);
    let res1 = prefix_checker(string1);
    let res2 = prefix_checker("HELLO");

    println!("res1 -> {res1}\nres2 -> {res2}");

    let status = "ACTIVE";
    let logger = get_boxed_logger(status);
    logger("System starting...");

    // Test Case 4
    let my_secret = String::from("Rust is awesome");
    let revealer = create_secret_revealer(&my_secret);

    // Even if we use 'revealer' here, it's valid because 'my_secret' is still alive.
    println!("Secret: {}", revealer());

    // ---------------------------------------------------------
    // SUMMARY OF RULES:
    // 1. impl Fn() + 'a: Means the closure captures something with lifetime 'a.
    // 2. for<'a> Fn(&'a str): Means the closure doesn't care about the lifetime
    //    until you actually call it.
    // 3. Box<dyn Fn() + 'a>: Required for trait objects to ensure the
    //    captured references don't expire while the Box is still being used.
    // ---------------------------------------------------------
}
