fn main() {
    let y = get_y();
    println!("Hello, Number {}!", y);

    let x = 4;
    // x is a basic integer type which implements copy trait.
    // drop does not work for types that implement copy trait.
    drop(x);
    println!("x = {}", x);

    let s = "hello".to_string();
    drop(s);
    println!("s = {}", s);
}

pub fn get_y<'a>() -> &'a i32 {
    let mut y = 13;
    let n = &mut y;
    *n += 3;
    &y
}