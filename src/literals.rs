use std::mem::size_of_val;

fn main() {
    // Suffixed literals, their types are known at initialization
    let x: u8 = 1;
    let y: u32 = 2;
    let z: f32 = 3.0;

    // Unsuffixed literals, their types depend on how they are used
    let i: i64 = 1;
    let f: f64 = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of \"x\" in bytes: {}", size_of_val(&x));
    println!("size of \"y\" in bytes: {}", size_of_val(&y));
    println!("size of \"z\" in bytes: {}", size_of_val(&z));
    println!("size of \"i\" in bytes: {}", size_of_val(&i));
    println!("size of \"f\" in bytes: {}", size_of_val(&f));
}