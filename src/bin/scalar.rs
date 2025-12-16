#![allow(unused)]

fn main() {
    //signed integers
    let i0: i8 = 0;

    let i1: i16 = 10;
    let i2: i32 = 100;
    let i3: i64 = 1000;
    let i4: i128 = 10000;
    let i5: isize = 0; // this depends on the computer architecture either 32 bits or 64 bits.

    //unsigned integers
    let u0: u8 = 0;

    let u1: u16 = 10;
    let u2: u32 = 100;
    let u3: u64 = 1000;
    let u4: u128 = 10000;

    //floats

    let f0: f32 = 0.0;
    let f1: f64 = 0.0;

    //bool

    let a: bool = true;
    let b: bool = false;

    //character

    let c: char = 'a';
    let d: char = 'b';

    //type conversion

    let i: i32 = 23;
    let u: u32 = i as u32;

    let a = u + (i as u32);

    println!("{}", a);
}
