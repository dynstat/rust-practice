// A 'trait' is a collection of methods that a type can implement. It's similar to an
// interface in other languages. Here, we define a `Logger` trait that requires any
// implementing type to have a `log` method. This allows us to write code that works
// with any kind of logger, as long as it adheres to this contract.
pub trait Logger {
    /// This is a documentation comment for the `log` method.
    /// It specifies that the method should log a message at a given verbosity level.
    // '&self' means the method borrows the instance it's called on, so it can't modify it.
    // 'verbosity: u8' is an 8-bit unsigned integer representing the log level.
    // 'message: &str' is a borrowed string slice for the log message.
    fn log(&self, verbosity: u8, message: &str);
}

// This is a simple struct that will act as our concrete logger.
// It's a "unit struct" because it has no fields. It's just a named type.
pub struct StderrLogger;

// This `impl` block provides the implementation of the `Logger` trait for the
// `StderrLogger` struct. This is how we fulfill the contract defined by the trait.
impl Logger for StderrLogger {
    // We provide the concrete implementation for the `log` method.
    fn log(&self, verbosity: u8, message: &str) {
        // `eprintln!` is a macro that prints the formatted string to the standard error stream.
        // This is a common practice for logging. The curly braces `{}` are placeholders
        // for the variables that follow.
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// This is another documentation comment, explaining what the `Filter` struct does.
// The `Filter` struct is a "wrapper" or "decorator". It takes one logger and adds
// filtering functionality to it.
// It is generic over two types, `L` and `P`. This makes it incredibly flexible.
// - `L` will represent the type of the logger that `Filter` wraps.
// - `P` will represent the type of the filtering logic (which will be a closure).
pub struct Filter<L, P> {
    // 'inner' will hold the logger that we are wrapping. Its type is the generic `L`.
    inner: L,
    // 'predicate' will hold the closure that decides whether to log a message or not.
    // Its type is the generic `P`. A predicate is just a function that returns true or false.
    predicate: P,
}

// This is an implementation block specifically for the `Filter` struct.
// The `<L, P>` after `impl` declares that this implementation is generic over the
// same `L` and `P` types as the struct definition.
impl<L, P> Filter<L, P>
// The `where` clause is crucial. It adds "trait bounds" or constraints to the
// generic types `L` and `P`. This implementation will only be valid if these
// constraints are met.
where
    // This constraint says "The type `L` must be a type that implements the `Logger` trait".
    // This is necessary because we need to call `self.inner.log(...)` later.
    L: Logger,
    // This constraint says "The type `P` must be a closure that can be called (`Fn`),
    // takes a `u8` and a `&str` as arguments, and returns a `bool`".
    // `Fn` is one of the three closure traits in Rust (`Fn`, `FnMut`, `FnOnce`).
    // `Fn` means the closure can be called multiple times without changing its captured state.
    P: Fn(u8, &str) -> bool,
{
    // This is a "constructor" function for our `Filter`. It's a common Rust convention
    // to have a `new` function to create instances of a struct.
    // The parameters `inner` and `predicate` have the generic types `L` and `P`.
    // The return type is `Self` (with a capital 'S'), which is an alias for the
    // type this `impl` block is for, i.e., `Filter<L, P>`.
    pub fn new(inner: L, predicate: P) -> Self {
        // This creates a new instance of the `Filter` struct.
        // The `new` function "knows" about the `inner` and `predicate` fields because
        // it is defined within the `impl Filter<L, P>` block.
        // `Self { inner, predicate }` is shorthand for `Self { inner: inner, predicate: predicate }`.
        // It initializes the struct's fields with the parameters of the same name.
        Self { inner, predicate }
    }
}

// This is a second `impl` block for `Filter`. This one makes the `Filter` struct
// itself a `Logger`. This is a powerful concept: our `Filter` can be used anywhere
// a `Logger` is expected.
impl<L, P> Logger for Filter<L, P>
// The `where` clause here is identical to the one above. We need the same constraints
// to be able to implement the `log` method.
where
    L: Logger,
    P: Fn(u8, &str) -> bool,
{
    // Here we implement the `log` method required by the `Logger` trait.
    fn log(&self, verbosity: u8, message: &str) {
        // The core logic of the filter.
        // We call the closure stored in `self.predicate`.
        // The syntax `(self.predicate)(...)` is used to invoke a closure that is stored in a field.
        if (self.predicate)(verbosity, message) {
            // If the predicate closure returns `true`, we then call the `log` method
            // on our inner logger, `self.inner`. We know `self.inner` has a `.log()`
            // method because of the `where L: Logger` constraint.
            self.inner.log(verbosity, message);
        }
        // If the predicate returns `false`, we do nothing, effectively filtering out the message.
    }
}
