#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
}

impl Triangle {
    pub fn new(p0: Point, p1: Point, p2: Point) -> Triangle {
        Triangle { p0, p1, p2 }
    }
}
