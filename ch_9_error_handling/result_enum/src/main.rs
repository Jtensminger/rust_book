#![allow(unused)]
use std::fmt::Error;
use std::fs::File;
use std::io::{self, Read};

fn main() {
        let greeting_file = File::open("hello.txt")?; // will error out because return type of main != Return Type
}

// fn main() {
//         fn read_username_from_file() -> Result<String, io::Error> {
//                 let mut username = String::new();

//                 File::open("hello.txt")?.read_to_string(&mut username)?;

//                 Ok(username)
//         }
// }

// fn main() {
//     let greeting_file_result = File::open("hello.txt");    

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
// }