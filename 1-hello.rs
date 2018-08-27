use std::io;
use std::io::Write;

fn main() {
    hello0();
    hello1();
    hello2();
    broken();
    println!("{}", hello3(format!("Cardenio")));
    println!("{}", hello4(format!("Dulcinea")));
}

/// To print, we grab a handle to stdout, and send it a sequence of bytes with
/// the write method.
fn hello0() {
    io::stdout().write(b"Hello reader\n");
}

/// We use let to assign values.
fn hello1() {
    let output = b"Hello reader\n";
    let mut out = io::stdout();
    out.write(output);
}

/// println!() is a macro (the ! tells us that), rather than an ordinary
/// function.  The reason it is a macro is because it accepts a more flexible
/// range of arguments than a normal Rust function can accommodate.
fn hello2() {
    println!("Hello Sancho Panza");
}

/// As in any language, a function can take arguments.  The type is annotated after
/// the parameter name.
fn hello3(input: String) -> String {
    format!("Hello {}", input)
}

// /// This function is broken because we try to use name after we have already passed
// /// ownership of `name` to `hello3`.  We can fix it by allocating a new String, or by
// /// returning ownership of the original String.
fn broken() {
    let name = "Cardenio".to_string();
    println!("{}", hello3(name.clone()));
    println!("We greeted {}", name);
}

/// Strings are growable vectors of valid UTF-8 bytes.  The data they contain
/// lives on the heap.  They can be allocated, modified, grown, and shrunk.
/// Generally they have more space allocated to them than they actually use,
/// so they don't need to be reallocated every time they are grown.
///
/// On the stack, a String consists of three pieces:
///
///     +--------------+       +---+---+---+---+---+---+---+---+---+---+---+---+
///     | data-ptr     | ----> | D | u | l | c | i | n | e | a | Uninitialized |
///     +--------------+       +---+---+---+---+---+---+---+---+---+---+---+---+
///     | length: 8    |
///     +--------------+
///     | capacity: 12 |
///     +--------------+
///
/// When Strings are declared to be mutable, they can be modified, expanded and
/// reallocated.
fn hello4(mut input: String) -> String {
    input.insert_str(0, "Señora ");
    input.push_str(" of Toboso");
    format!("Hello {}", input)
    // input.insert_str("Hello ");
    // input
}
