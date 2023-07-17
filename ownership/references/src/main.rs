fn main() {
    references_and_borrowing();
    mutable_references();
    // dangle();
    no_dangle();
}


fn references_and_borrowing() {
    let s1: String = String::from("hello");

    let len: usize = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what

// mutable references
fn mutable_references() {
    let mut s: String = String::from("hello");

    change(&mut s);

    // race condition
    // let r1: &mut String = &mut s;
    // let r2: &mut String = &mut s;

    // only one mutable reference
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
