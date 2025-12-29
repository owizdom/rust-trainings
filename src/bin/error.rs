#![allow(unused)]

fn get_age(name: &str) -> Option<u8> {
    match name {
        "Alice" => Some(30),
        "Bob" => Some(25),
        _ => None, // user not found
    }
}

fn parse_number(input: &str) -> Result<i32, String> {
    input
        .parse::<i32>()
        .map_err(|_| "Invalid number".to_string())
}

fn main() {
    let age = get_age("Bob");

    match age {
        Some(a) => println!("Age is {a}"),
        None => println!("User not found"),
    }

    let result = parse_number("7");

    match result {
        Ok(n) => println!("Parsed number: {n}"),
        Err(e) => println!("Error: {e}"),
    }
}
