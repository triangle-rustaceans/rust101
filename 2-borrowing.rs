
fn main() {
    let shepherdess = format!("Marcela");

    println!("{}", hello5(&shepherdess));

    println!("We just said hello to the shepherdess {}", shepherdess);
    println!("{}", hello5(&shepherdess));
    let r = &shepherdess;
    println!("{}", hello5(r));
    println!("{}", hello5(r));

    println!("{}", hello_both(&shepherdess, r));

    let name = format!("Señora Dulcinea del Toboso");
    let dulcinea = &name[8..16];

    // Broken!
    // println!("{}", hello5(&dulcinea));
    println!("{}", hello6(&dulcinea));

    // This is okay because String can automatically dereference to &str.
    println!("{}", hello6(&name));
}

/// If we take a reference to a String instead of an owned String object, the
/// calling function can continue to use it after it has been passed to
/// can continue to use the object after we have passed it to the calling
/// function.
fn hello5(input: &String) -> String {
    format!("Hello {}", input)
}

/// In fact, we can also have multiple references to the same object at the
/// same time, but only if we are not mutating the object during the lifetime
/// of the references.
fn hello_both(input1: &String, input2: &String) -> String {
    format!("Hello {} and {}!", input1, input2)
}

// /// The following fails to compile because it tries to modify the same object
// /// it is taking a reference to while the reference is alive:
// fn modify_self() {
//     let mut s = format!("Giants");
//     let giants = &s;
//     s.clear();
//     s.push_str("Just windmills");
//     println!("{}", giants);
// }

/// &str is a reference to a string slice.  It consists of a pointer to
/// existing string data, either in a String object, or baked into the
/// program's data section.
///
/// On the stack this looks like:
///
///     +----------+
///     | data-ptr |
///     +----------+
///     | length   |
///     +----------+
///
/// Note that our &str does not have access to the capacity of the String.
/// It also cannot directly modify its length.
fn hello6(input: &str) -> String {
    format!("hello {}", input)
}
