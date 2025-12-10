use crate::point::Point;

pub struct Rectangle {
    pub point_a: Point,
    pub point_b: Point,

    pub width: f64,
    pub height: f64,

    pub area: f64,
}

impl Rectangle {
    pub fn new(a: Point, b: Point) -> Self {
        let width = 1.0 + (a.x - b.x).abs();
        let height = 1.0 + (a.y - b.y).abs();

        let area = width * height;

        Rectangle {
            point_a: a,
            point_b: b,
            width,
            height,
            area,
        }
    }

    pub fn print(&self) {
        println!(
            "Rectangle: {},{}\nWidth: {}, Height: {}\n Area: {}",
            self.point_a, self.point_b, self.width, self.height, self.area
        );
    }
}
