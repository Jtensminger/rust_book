/*  Task: Convert strings to pig latin.

[R1]: if s[0] = consonant, Move Consonant to end & append "-ay"
>>> The first consonant of each word is moved to the end of the word &
>>> “ay” is added, so “first” becomes “irst-fay.”

[R2]:  if s[0] = vowel, Append "-hay" 
>>> Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).

[Reminder]: Keep in mind the details about UTF-8 encoding!
*/

use std::collections::HashMap;
fn main () {
    // Take in a word
    // let mut word: String = String::from("apple");
    let mut word: String = String::from("first");

    // Find the first letter & determine which rule to apply
    let initial_letter = initial_letter(&mut word);
    let rule = classify_word(&initial_letter);
    
    // Define Operation
    let mut operation = PigLatinOperation {
        rule: rule,
        target_word: word,
    };

    // Apply Operation
    let translation = match &operation.rule {
        PigLatinRule::VOWEL => vowel_transform(&mut operation),
        PigLatinRule::CONSONANT => consonant_transform(&mut operation)
    };

    println!("Pig latin: {}", translation);
}
enum PigLatinRule {
    VOWEL,
    CONSONANT
}
struct PigLatinOperation {
    rule: PigLatinRule,
    target_word: String,
}

// Find the first letter
fn initial_letter (original_word: &mut String) -> char {
    let mut chars = original_word.chars();
    chars.next().expect("err: empty String")
}

// Determine which rule to apply
fn classify_word (first_letter: &char) -> PigLatinRule {
    
    // character lookup table
    let vowels = HashMap::from([
        ('a', PigLatinRule::VOWEL),
        ('A', PigLatinRule::VOWEL),
        ('e', PigLatinRule::VOWEL),
        ('E', PigLatinRule::VOWEL),
        ('i', PigLatinRule::VOWEL),
        ('I', PigLatinRule::VOWEL),
        ('o', PigLatinRule::VOWEL),
        ('O', PigLatinRule::VOWEL),
        ('u', PigLatinRule::VOWEL),
        ('U', PigLatinRule::VOWEL),
        ('y', PigLatinRule::VOWEL),
        ('Y', PigLatinRule::VOWEL),
    ]);
    
    // see if character is in vowel lookup table & return type
    match vowels.contains_key(first_letter) {
        true => PigLatinRule::VOWEL,
        false => PigLatinRule::CONSONANT,
    }
}

// Apply conditional transformations
fn vowel_transform (operation: &mut PigLatinOperation) -> String {
    let mut new_word = operation.target_word.clone();
    new_word.push_str("-hay");
    
    return new_word
}

fn consonant_transform (operation: &mut PigLatinOperation) -> String {
    // Get individual letters
    let mut letters = operation.target_word.chars();
    
    // Remove 1st letter
    letters.next();

    // Construct the new word
    let mut new_word = String::new();
    for c in letters {
        new_word.push(c);
    }

    new_word.push('-');
    new_word.push(initial_letter(&mut operation.target_word));
    new_word.push_str("ay");
    
    return new_word
}



/*
////// Unicode is a set of valid Characters //////
>> Character: has semantic value
>> Coded Character: is a symbol's corresponding unique numeric value, an integer in Unicode
>> Code Space: range of valid integers (unique numeric values), each integer called a Coded Character
//////////////////////////////////////////////////

////// Unicode is a set of valid Characters //////
>> UTF (Unicode Transformation Format) encoding standard creates the valid Code Space & Code Point per Character
>> Multiple UTF standards exist, each creating different Code Unit sizes (UTF-8, UTF-16, UTF-32, # = bits)
//////////////////////////////////////////////////



////// ?????????? IDK ??????????? //////
>> Code Units: the "word size" of the numbers in the encoding standard (8-bits, 16-bits, 32-bits)
>> Code Point: is a sequence of Code Units
//////////////////////////////////////////////////




>> UTF-8 is "variable width", which means a Code Point contains between 1 — 4 bytes
>>>>>>>> 1 Byte is 8 bits
>>>>>>>> 2 Bytes is 16 bits
>>>>>>>> 3 Bytes is 24 bits
>>>>>>>> 4 Bytes is 32 bits
>>>>>>>> Any UTF-8 Code Point could contain between 8 - 32 bits
>>>>>>>> Which i32, i64, isize, usize has enough space to store any UTF-8 Code Point

>> “नमस्ते” String of Devangari script
>> Code Space: U+0000 to U+10FFFF
>> Code Units: 
>> Code Points:
>> 




/////////////////////////////////////////////////


*/
























//////////////////////////////////////////////////////////////////////////////////////////////
// Given a list of integers, 
// use a vector and return the median (when sorted, the value in the middle position) 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

// use std::{collections::HashMap, vec};
// fn main() {
//     let mut raw_list: Vec<i32> = vec![12, 474, 4, 1, 7, 39, 8, 55];
    
//     println!("median: {}", median(&mut raw_list));

//     match mode(&raw_list) {
//         Some(v) => println!("mode: {}", v),
//         None => println!("mode: none")
//     }
// }

// fn median(vec: &mut Vec<i32>) -> i32 {
//     vec.sort();

//     match vec.len() % 2 {
//         0 => (vec[vec.len() / 2] + vec[(vec.len() - 1) / 2]) / 2,
//         _ => vec[(vec.len() - 1) / 2]
//     }
// }

// fn mode(vec: &Vec<i32>) -> Option<i32> {
//     let mut hm: HashMap<i32,i32> = HashMap::new();
//     let mut mode: i32 = 0;
//     let mut highest_count: i32 = 1;
//     let list_length: usize = vec.len();

//     for item in vec {
//         let count = hm.entry(*item).or_insert(0);
//         *count += 1;

//         if *count > highest_count {
//             mode = *item;
//             highest_count = *count;
//         }
//     }

//     match hm.keys().len() != list_length {
//         true => Some(mode),
//         false => None
//     }
// }
//////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////
//  Playing w/ Hash Maps
// 
// fn main() {
//     let mut h: HashMap<char, Vec<usize>> = HashMap::new();

//     for (i, c) in "hello!".chars().enumerate() {
//         h.entry(c)
//         .or_insert(Vec::new())
//         .push(i);
//     }

//     let mut sum = 0;

//     for i in h.get(&'l').unwrap() {
//         sum += *i;
//     }

//     println!("{}", sum);
// }
//////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////
// Hash Map basics
// 
// fn main () {
//     // initializing a Hash Map
//     let mut scores: HashMap<String, i32> = HashMap::new();

//     // adding k,v pairs to the initialized Hash Map
//     scores.insert(String::from("Yellow"), 50);

//     // Retrieving a reference to a value from a given key
//     let score = scores.get(&String::from("Blue")).unwrap_or(&0);

//     // adding a key value pair if the key doesn't already exist
//     let mut blue_score = scores.entry(String::from("Blue")).or_insert(50);
//     *blue_score += 5;
//     println!("{}", blue_score);

//     // iterate over each key/value pair in a Hash Map
//     for (key, value) in &scores {
//         println!("{}: {}", key, value)
//     }

// }
//////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////
// Playing w/ Strings
// 
// fn main () {
//     let s1 = String::from("नHello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2[..2] + " woah brah"; // note s1 has been moved here and can no longer be used

//     // indexing w/ a range to create a string slice containing particular bytes:
//     println!("s3 index 0 is {}", &s3[0..3]);

//     // Iterating Over Strings of Unicode Scalar Values that return chars
//     for c in s3.chars() {
//         println!("{}", c);
//     }

//     // Iterating Over Strings of Unicode Scalar Values that return bytes
//     for b in s3.bytes() {
//         println!("{}", b);
//     }

// }

//////////////////////////////////////////////////////////////////////////////////////////////

// Using enums to store multiple types and iterating through them
// fn main() {

//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];


//     for n_ref in &row {
//         match n_ref {
//             SpreadsheetCell::Int(v) => println!("{}", v),
//             SpreadsheetCell::Text(v) => println!("{}", v),
//             SpreadsheetCell::Float(v) => println!("{}", v),
//         }
//     }
// }


// Iterating and changing values in a vector
// fn main() {
//     let mut v = vec![100, 32, 57];
//     for n_ref in &mut v {
//         // n_ref has type &mut i32
//         *n_ref += 50;
//     }
// }


// Iterating over the values in a vector
// fn main() {
//     let v = vec![100, 32, 57];
//     for n_ref in &v {
//         let n_plus_one: i32 = *n_ref + 1;
//         println!("{}", n_plus_one);
//     }
// }


// Accessing Elements in a Vector via get() & indexing
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);

//     let third: Option<&i32> = v.get(2);

//     if let Some(third) = third {
//         println!("The third element is {}", third);
//     }
// }
