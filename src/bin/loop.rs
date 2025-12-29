#![allow(unused)]

fn main() {
    //while loop
    let mut x = 0;

    while x < 5 {
        println!("x = {x}");
        x += 1;
    }

    //index
    for i in 0..5 {
        println!("{i}");
    }

    //array
    let nums = [10, 20, 30];

    for n in nums {
        println!("{n}");
    }

    //for vector
    let v = vec![2, 3, 4];

    for x in v {
        println!("vector {x}")
    }

    //use iter to make vector loop loops twice.
    //
    'outer: for a in 0..10 {
        'inner: for b in 0..10 {
            println!("{a}, {b}");
            if a == 6 && b == 7 {
                break 'outer;
            }
        }
    }
}
