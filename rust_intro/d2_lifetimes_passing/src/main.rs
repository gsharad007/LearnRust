fn main() {
    let mut x = 7;
    let mut y = 15;
    let pt = choose(&mut x, &mut y);
    *pt += 2;

    println!("x or y => pt {}", *pt);
    println!("x or y => pt {}", *pt);
    x -= 2;
    y -= 2;
    println!("x {} or y {}", x, y);
    
    let th = make_thing(&x, y);
    println!("Thing = {:?}", th);
    y += 2;
    x += 2;
    println!("x {} or y {}", x, y);
}

fn choose<'a>(x: &'a mut i32, y: &'a mut i32) -> &'a mut i32 {
    if rand::random() {
        x
    } else {
        y
    }
}

pub fn make_thing<'a>(a: &'a i32, b: i32) -> Something<'a> {
    Something { pt: a, age: b }
}

#[derive(Debug)]
pub struct Something<'a> {
    pt: &'a i32,
    age: i32,
}
