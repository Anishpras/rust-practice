fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

fn main() {
    let (x, y) = (1, 2);
   let x:i32 = x+2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}