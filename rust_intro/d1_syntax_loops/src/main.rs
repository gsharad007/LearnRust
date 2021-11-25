use std::env::args;

fn main() {
    for (i, x) in args().enumerate().skip(1) {
        println!("{}: Hello {}!", i, x);
    }
    for x in 1..=10 {
        println!("{}", x);
    }
    for (k, v) in (10..20).enumerate() {
        println!("({}, {})", k, v);
    }
    for (i, x) in Fibonacci::new().take(25).enumerate() {
        println!("{}: {}", i, x);
    }
    let mut i = Fibonacci::new().filter(|x| x % 2 == 0).take(25).enumerate();
    while let Some((k, v)) = i.next() {
        println!("Other fib {}: {}", k, v);
    }
}

struct Fibonacci {
    a: i64,
    b: i64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci{
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a + self.b;
        self.a = self.b;
        self.b = c;
        Some(self.a)
    }
}
