use std::ops::Add;

#[derive(Debug)]
struct AVec<T>(Vec<T>);

impl<T> Add for AVec<T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.0.extend(rhs.0.into_iter());
        self
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![11, 12, 13, 14, 15];
    let v3 = AVec(v1) + AVec(v2);

    println!("v3 = {:?}", v3);
}
