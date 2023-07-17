fn main() {
    let mut s = String::from("hello world");
    // let hello = &s[..5]; // slice of first 5 bytes
    // let world = &s[..]; // slice of all bytes

    // let s2: &str = &s; // immutable reference to String

    let word = first_word(&s); // word will get the value 5, immutable reference
    println!("the first word is: {}", word);

    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    array_slice();
}


fn first_word(s: &String) -> &str { // s is a reference to a String
    let bytes = s.as_bytes(); // convert String to array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterate over array of bytes
        if item == b' ' { // if byte is a space
            return &s[..i]; // return slice of string up to space
        }
    }

    &s[..] // return slice of entire string
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5]; // [i32, _]
    let slice = &a[1..3]; // slice of array &[i32]
    println!("slice: {:?}", slice);
}
