#[derive(Debug, PartialEq, Clone)]
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
pub enum Action {
    Shot { from: Point, at: Point },
    Move { from: Point, to: Point },
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
    pub fn time_step(&mut self, v: &mut Vec<Action>) {
        use std::cmp::Ordering::*;
        match &self.status {
            ShipStatus::Heading(p) => {
                let nx = match p.x.cmp(&self.location.x) {
                    Greater => p.x + 1,
                    Less => p.x - 1,
                    Equal => p.x,
                };
                let ny = match p.y.cmp(&self.location.y) {
                    Greater => p.y + 1,
                    Less => p.y - 1,
                    Equal => p.y,
                };
                v.push(Action::Move {
                    from: self.location.clone(),
                    to: Point::new(nx, ny),
                });
                self.location = Point::new(nx, ny);
            }
            ShipStatus::Firing(p) => v.push(Action::Shot {
                from: self.location.clone(),
                at: p.clone(),
            }),
            _ => {}
        }
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

    let mut actions: Vec<Action> = Vec::new();
    ship.time_step(&mut actions);
    println!("Actions = {:?}", actions);

    if let ShipStatus::Firing(x) = ship.status {
        println!("This ship is firing at {:?}", x);
    }
}
