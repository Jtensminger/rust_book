use std::mem::size_of;


fn main() {
    println!("size_of: {} bytes", size_of::<(char, u8, i32)>());
}

