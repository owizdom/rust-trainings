#![allow(unused)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    fav_color: String,
}

fn main() {
    let test = "James";
    println!("Hello, {test}");

    let x = 3;
    println!("{0} x {0} = {1}", x, x * x);
    println!("{0} x {0} x {0} = {1}", x, x * x * x);

    let person = Person {
        name: String::from("John"),
        age: 30,
        fav_color: String::from("Blue"),
    };

    println!("{:#?}", person);
}
