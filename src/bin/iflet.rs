#![allow(unused)]

fn main() {
    let option: Option<u64> = Some(45); // None -> the output will be "None"
    match option {
        Some(i) => println!("matched {i}"),
        None => println!("None"),
    }

    //if let
    if let Some(v) = option {
        println!("if let {v}");
    }

    //let else
    let Some(v) = option else {
        panic!("x is none");
    };
    println!("let else {v}");
}
