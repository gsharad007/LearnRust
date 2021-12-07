use std::fmt::Debug;
use std::fmt::Display;

trait TimeTrait: Debug + Display {}
impl<T: Debug + Display> TimeTrait for T {  }

fn main() {
    let mut v: Vec<Box<dyn TimeTrait>> = Vec::new();
    v.push(Box::new(3));
    v.push(Box::new("Hello"));

    println!("v = {:?}", v);
}
