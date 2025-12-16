#![allow(unused)]

fn main() {
    //creating tuple
    let t: (bool, u8, char) = (true, 255, 'a');

    //destructure
    let (a, b, c) = t;

    //empty tuple
    let t = ();

    //nested
    let nested = ((2, 'c'), (444, "obi"), ());

    println!("nested {}, {}", nested.0.0, nested.1.1);
}
