fn add(x: i32, y: i32) -> i32 {
    x + y
}

static A: &str = "Hello";

fn forever() -> ! {
    loop {}
}

fn crash() -> ! {
    panic!("Crash!");
}

fn main() {
    let result = add(3, 5);
    println!("The result is {}", result);

    println!("{}", A);

    forever();
}
