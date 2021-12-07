use std::io::{Read, Write};
use thiserror::*;

#[derive(Error, Debug, PartialEq)]
enum BError {
    #[error("No Target Provided")]
    NoTarget,
    #[error("Divide by zero")]
    DivError,
}

fn main() -> anyhow::Result<()> {
    let mut it = std::env::args().skip(1);
    let target = it.next().ok_or(BError::NoTarget)?;
    let mut tot = 0;
    let mut count = 0;
    let mut target = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(target)?;
    for a in it {
        let s = sum_file(&a)?;
        tot += s;
        count += 1;
        writeln!(target, "{: <25} = {}", a, s)?;
    }

    if count == 0 {
        return Err(BError::DivError.into());
    }

    writeln!(target, "----------------")?;
    writeln!(target, "Avg = {}", tot / count)?;
    println!("DONE");

    Ok(())
}

// fn sum_file(fname:&str) -> Result<isize, Box<dyn error::Error>> {
fn sum_file(fname: &str) -> anyhow::Result<isize> {
    let mut s = String::new();
    std::fs::File::open(fname)?.read_to_string(&mut s)?;
    let mut res = 0;
    for n in s.trim().split("\r\n") {
        println!("n = {:?}", n);
        res += n.parse::<isize>()?;
    }
    Ok(res)
}
