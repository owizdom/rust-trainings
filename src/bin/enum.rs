#![allow(unused_variables, unused_parens, unused)]

fn main() {
    //enum
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Red;
    match color {
        Color::Red => println!("This match is Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    let color = Color::Green;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("This match is Green"),
        Color::Blue => println!("Blue"),
    }

    let color = Color::Blue;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("This match is Blue"),
    }

    //option

    let x: Option<i32> = None;
    let x: Option<i32> = Some(89);
    println!("option: {:?}", x);

    //result

    let res: Result<u32, String> = Ok((5));
    let res: Result<u32, String> = Err("div by 0".to_string());
    println!("result: {:?}", res);
}
