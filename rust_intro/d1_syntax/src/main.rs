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
    println!("ship = {:?}", ship);
}
