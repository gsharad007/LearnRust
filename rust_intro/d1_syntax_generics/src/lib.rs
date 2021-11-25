use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Clone)]
pub struct Point<N> {
    x: N,
    y: N,
}

impl<N> Point<N> {
    pub fn new(x: N, y: N) -> Self {
        Point { x, y }
    }
}
impl<N: AddAssign + Clone> Point<N> {
    pub fn transpose(&mut self, p: &Self) {
        self.x += p.x.clone();
        self.y += p.y.clone();
    }
}
impl<N: Add<Output = N> + Clone> Add for Point<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn adding_point_i32() {
        let p1 = Point::new(100, 100) + Point::new(100, -100);
        assert_eq!(Point::new(200, 0), p1);
    }

    #[test]
    fn transposing_point_i32() {
        let mut p1 = Point::new(100, 100);
        p1.transpose(&Point::new(100, -100));
        assert_eq!(Point::new(200, 0), p1);
    }
}
