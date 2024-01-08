fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;
  let z:i32 = 10;
    println!("Success!");
}

fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}
fn main() {
    let x:u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

fn main() {
    let v1 = 251_i16 + 8;
    let v2:i16 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }
 fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}
fn main() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn main() {
    assert!(0.1 as f32 +0.2 as f32 ==0.3 as f32);
 
    println!("Success!");
}
fn main() {
    assert!(0.1_f32 +0.2_f32 ==0.3_f32);
 
    println!("Success!");
}

// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}