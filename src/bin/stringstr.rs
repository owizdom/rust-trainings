#![allow(unused_variables)]

fn main() {
    //string

    let msg: String = String::from("Hello, world!");
    let len: usize = msg.len();
    println!("Length: {}", len);
    println!("Message: {}", msg);

    //string slice

    let slice: &str = &msg[0..5];
    println!("The Slice is {slice}");

    let mut new_slice: String = "Hello".to_string();
    new_slice += "!";
    println!("{new_slice}");
}
