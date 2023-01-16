

// enum Location {
//     Point(i32),
//     Range(i32, i32)
// }

// fn print_range_max(loc: &Location) {
//     // print the second field of Range, if loc is a Range
//     // match loc {
//     //     Location::Range(x,y) => println!("{}", y),
//     //     _ => (),
//     // }

//     if let Location::Range(x,y) = loc {
//         println!("{}", y)
//     }
// }
// fn main() {
//     print_range_max(&Location::Range(1,2));
// }

// --------- Concise Control Flow with if let ---------

// fn main() {
//     let config_max = Some(3u8);

//     // both code blocks below are equivalent
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {}", max),
//         _ => (),
//     }

//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {}", max);
//     }
// }

// ------- A function that uses a match expression on an Option<i32> ---------
// Let’s say we want to write a function that takes an Option<i32> and,
// if there’s a value inside, adds 1 to that value. 
// If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.

// fn plus_one(option: Option<i32>) -> Option<i32> {
//     match option {
//         None => None,
//         Some(option_int) => Some(option_int + 1),
//     }
// }


// fn main() {
//     println!("{:?}", plus_one(Option::from(5)) );
// }



// Playing with Enums, variants, & Enum methods
// fn main() {
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }

//     impl Message {
//         fn call(&self) {
//             // method body would be defined here
//         }
//     }

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }
