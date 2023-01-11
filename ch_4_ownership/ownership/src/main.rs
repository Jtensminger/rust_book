// write a function that takes a string of words separated by spaces &
// returns the first word it finds in that string


// what are bytes? what's the keyword bytes?

fn main() {
    let words: String = String::from("first second third");
     
    let first_word: &str = get_first_word(&words);

    println!("{}", first_word);
}

fn get_first_word(s: &String) -> &str { // input: String Reference output: str slice reference
    
    let bytes: &[u8] = s.as_bytes(); // converts a string slice into an [u8] where each u8 represents a character.

    for (byte_index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' { return &s[..byte_index] }
    }

    return &s[0..2]; // returns a string slice
}


// OWNERSHIP
// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.
