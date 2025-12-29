#![allow(unused)]

enum Animal {
    Cat,
    Duck,
    Goat,
}

fn main() {
    let number = 2;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),
    }

    match number {
        1 | 4 | 3 => println!("either 1 or 2 or 3"),
        _ => println!("some other"),
    }

    match number {
        i @ 1..20 => println!("matched {i}"),
        _ => println!("okkay, others"),
    }

    let animal = Animal::Duck;
    let animal_sound = match animal {
        Animal::Cat => println!("meow"),
        Animal::Duck => println!("quack"),
        Animal::Goat => println!("blehh"),
        _ => println!("unknown"),
    };

    let option: Option<u64> = None;
    match option {
        Some(i) => println!("matched {i}"),
        None => println!("None"),
    }

    let res: Result<i32, String> = Err("45".to_string());
    match res {
        Ok(i) => println!("matched {i}"),
        Err(e) => println!("Error: {e}"),
    }
}
