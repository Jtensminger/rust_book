// Playing w/ Strings

fn main () {
    let s1 = String::from("नHello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2[..2] + " woah brah"; // note s1 has been moved here and can no longer be used

    // indexing w/ a range to create a string slice containing particular bytes:
    println!("s3 index 0 is {}", &s3[0..3]);

    // Iterating Over Strings of Unicode Scalar Values that return chars
    for c in s3.chars() {
        println!("{}", c);
    }

    // Iterating Over Strings of Unicode Scalar Values that return bytes
    for b in s3.bytes() {
        println!("{}", b);
    }

}


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
