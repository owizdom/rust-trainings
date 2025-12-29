#![allow(unused_variables)]

fn main() {
    //arrays
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("Numbers: {:?}", numbers);

    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits: {:?}", fruits);
    println!("Fruits: {:?}", fruits[0]);

    //tuples
    let tuple = ("hello", 42, 3.14);
    println!("Tuple: {:?}", tuple);

    let new_tuple = ("hello world", 67, true, numbers[0]);
    println!("New Tuple: {:?}", new_tuple);

    //slices:
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("Slice: {:?}", slice);

    //strings
    let string = "Hello, world!";
    println!("String: {}", string);
}
