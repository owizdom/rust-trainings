#![allow(unused)]

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10, 2).unwrap();
    println!("result = {result}");

    let x: Option<i32> = None;
    let value = x.expect("x should never be None");
}
