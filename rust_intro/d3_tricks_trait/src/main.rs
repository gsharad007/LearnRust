use std::ops::Add;

struct Doubler<I: Iterator> {
    inner: I,
}

impl<I: Iterator<Item = O> + Sized, O: Clone + Add> Iterator for Doubler<I> {
    type Item = O::Output;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|o| o.clone() + o)
    }
}

trait Double: Iterator + Sized {
    fn double(self) -> Doubler<Self> {
        Doubler { inner: self }
    }
}

impl<I: Iterator<Item = O> + Sized, O: Clone + Add> Double for I {}

fn main() {
    for i in (0..10).double() {
        println!("i {:?}", i);
    }
    println!("Done!");
}
