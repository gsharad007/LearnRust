#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Ship {
    location: Point,
    status: ShipStatus,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn transpose(&mut self, p: &Point) {
        self.x += p.x;
        self.y += p.y;
    }
}

impl Ship {
    pub fn attack(&mut self, p: Point) {
        self.status = ShipStatus::Firing(p);
    }
}

#[derive(Debug, PartialEq)]
pub enum ShipStatus {
    Engaged,
    Waiting,
    Firing(Point),
    Heading(Point),
}

fn main() {
    let mut ship = Ship {
        location: Point { x: 10, y: 10 },
        status: ShipStatus::Waiting,
    };
    ship.location.x += 10;
    println!("ship location = ({}, {})", ship.location.x, ship.location.y);
    println!("Ship Waiting = {:?}", ship);

    let mut a = Point::new(10, 4);
    let b = Point::new(20, -4);
    a.transpose(&b);
    assert_eq!(a, Point::new(30, 0));

    ship.attack(b);
    println!("Ship Attacking = {:?}", ship);
}
