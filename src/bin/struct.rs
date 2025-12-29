#![allow(unused)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

struct Point3D(f32, f32, f32);

struct EmptyStruct;

struct Unknown {
    center: Person,
    radius: u32,
}

fn main() {
    let test = Person {
        name: "James".to_string(),
        age: 34,
        email: "oke@gmail.com".to_string(),
    };

    println!("{:?}", test);
}
