fn main() {
    // --- Ownership Rules ---
    // Rule 1: Each value in Rust has a variable that’s called its owner.
    // Rule 2: There can only be one owner at a time.
    // Rule 3: When the owner goes out of scope, the value will be dropped.

    rule_1();
    copy_move_ownership();
    string_vs_int();
    ownership_and_functions();
}

fn rule_1() {
    // Rule 1: Each value in Rust has a variable that’s called its owner.
    // The variable is in scope.

    // >>> let s: &str = "hello"; //stack allocated
    let s = String::from("hello"); //heap allocated


    // Here, s is the owner of the String "hello".
    // This is the first rule of ownership.
    // When the owner goes out of scope, the value will be dropped.
    // This is the third rule of ownership.
    println!("{}", s);
}

fn copy_move_ownership() {
    let x: i32 = 5;
    let y: i32 = x; // copy, not move

    println!("x = {}, y = {}", x, y);

    let s1: String = String::from("hello");
    let s2: String = s1; // move ( s1 is no longer valid, not shallow copy )

    // println!("{}", s1); // error: value borrowed here after move

    let s3: String = s2.clone(); // deep copy

    println!("{}", s2);
    println!("{}", s3);
}

// string vs int example
fn string_vs_int() {
    let s: String = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x: i32 = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn ownership_and_functions() {
    let s1: String = gives_ownership(); // gives_ownership moves its return
    let s2: String = String::from("hello"); // s2 comes into scope
    let s3: String = takes_and_gives_back(s2); // s2 is moved into
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn gives_ownership() -> String { // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string: String = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
