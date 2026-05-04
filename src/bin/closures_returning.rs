//  take a input variable and return a closure that uses it
fn create_closure_that_adds(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn create_closure_that_appends_once(x: String) -> impl FnOnce(String) -> String {
    move |y| x + &y
}
fn create_closure_that_appends_many(x: String) -> impl Fn(String) -> String {
    move |y| x.clone() + &y
}

fn main() {
    let add_100 = create_closure_that_adds(100);
    // now this add_100 closure can be used to add 100 to any number
    println!("{}", add_100(50));

    let append_once_to_yo = create_closure_that_appends_once("yo".to_string());
    println!("{}", append_once_to_yo(" whats up ?".to_string()));
    // println!("{}", append_once_to_yo(" whats up ?".to_string())); // This will cause ERROR due to FnOnce
    let append_many_to_hi = create_closure_that_appends_many("hi,".to_string());
    println!("{}", append_many_to_hi(" i am Vivek".to_string()));
    println!("{}", append_many_to_hi(" i am Vivek".to_string()));
}
