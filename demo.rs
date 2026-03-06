pub trait MyTrait {
    fn do_work(&self) -> i32;
}

impl MyTrait for i32 {
    fn do_work(&self) -> i32 {
        *self * 2
    }
}

impl MyTrait for f64 {
    fn do_work(&self) -> i32 {
        *self as i32 * 3
    }
}

// 1. Generic version (Monomorphization)
#[inline(never)] // Prevents the compiler from inlining the function, ensuring that each monomorphized version is a distinct call in the binary.
pub fn process_generic<T: MyTrait>(val: T) -> i32 {
    val.do_work()
}

// 2. Trait Object version (Dynamic Dispatch)
#[inline(never)]
pub fn process_dynamic(val: &dyn MyTrait) -> i32 {
    val.do_work()
}

pub fn main() {
    // Compile-time expansion (generics)
    let a = process_generic(10_i32);
    let b = process_generic(10.0_f64);

    // Runtime dispatch (trait objects)
    let c = process_dynamic(&10_i32);
    let d = process_dynamic(&10.0_f64);

    // Black box to prevent aggressive removal
    std::hint::black_box((a, b, c, d));
}
