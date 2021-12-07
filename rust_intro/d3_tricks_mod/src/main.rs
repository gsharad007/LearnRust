mod hello;
mod line;
use d1_syntax_generics::Point;

fn main() {
    hello::hello();
    let p1 = Point::new(1.0, 1.0);
    let p2 = Point::new(4.0, 5.0);
    println!("distance: {}", line::dist(&p1, &p2));
    local::do_ya_thing();
}

mod local {
    pub fn do_ya_thing() {
        println!("thing done!");
    }
}