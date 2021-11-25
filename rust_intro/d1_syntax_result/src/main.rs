use std::env::args;
use std::fs::File;
// use std::io;
use std::io::Read;

// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

#[derive(Debug)]
pub enum MyErr {
    IOErr(std::io::Error),
}

impl From<std::io::Error> for MyErr {
    fn from(err: std::io::Error) -> Self {
        MyErr::IOErr(err)
    }
}

fn main() {
    for a in args().skip(1) {
        let w = count_ws(&a);
        let w2 = count_ws2(&a);
        println!("{}, has {:?} or {:?}", a, w, w2);
        if let (Ok(a), Ok(b)) = (&w, &w2) {
            assert_eq!(a, b);
            println!("Same!!!");
        }
    }
    println!("Hello, world!");
}

pub fn count_ws(filename: &str) -> Result<i32, MyErr> {
    let mut f = match File::open(filename) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let mut buf = String::new();
    if let Err(e) = f.read_to_string(&mut buf) {
        return Err(MyErr::IOErr(e));
    }

    let count = buf.chars().filter(|c| *c == 'w').count() as i32;

    // for c in buf.chars() {
    //     if c == 'w' {
    //         count += 1;
    //     }
    // }

    Ok(count)
}

pub fn count_ws2(filename: &str) -> Result<i32, MyErr> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s.chars().fold(0, |n, c| if c == 'w' { n + 1 } else { n }))
}
