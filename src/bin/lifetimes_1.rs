// ---------------------------------------------
// Struct holding reference
// ---------------------------------------------
struct Holder<'a> {
    data: &'a str, // 'a = lifetime of referenced data
}

// ---------------------------------------------
// Implementation
// ---------------------------------------------
impl<'a> Holder<'a> {
    // Fully expanded form:
    // fn get<'b>(&'b self) -> &'b str
    fn get(&self) -> &str {
        // self: &'b Holder<'a>
        // self.data: &'a str

        // We return &'b str (due to elision rule)
        // This works because: 'a: 'b (data lives longer than borrow)

        self.data
    }
}

// ---------------------------------------------
// MAIN FUNCTION (actual execution)
// ---------------------------------------------
fn main() {
    // -----------------------------------------
    // Step 1: create owned data
    // -----------------------------------------
    let s: String = String::from("hello world");
    // s owns memory

    // -----------------------------------------
    // Step 2: create Holder with reference
    // -----------------------------------------
    let h: Holder = Holder {
        data: &s, // &'a str → 'a tied to s's lifetime
    };

    // -----------------------------------------
    // Step 3: call method
    // -----------------------------------------
    let r: &str = h.get();
    //        ^^^^^^^^
    // r gets lifetime 'b (borrow of h during call)

    println!("r = {}", r);

    // -----------------------------------------
    // Step 4: use again
    // -----------------------------------------
    let r2: &str = h.get();
    println!("r2 = {}", r2);
}
