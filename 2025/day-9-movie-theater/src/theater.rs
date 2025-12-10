use crate::point::Point;
use crate::rectangle::Rectangle;

pub struct Theater {
    corners: Vec<Point>,
    rectangles: Vec<Rectangle>,
}

impl Theater {
    pub fn new() -> Self {
        Theater {
            corners: Vec::new(),
            rectangles: Vec::new(),
        }
    }

    pub fn rectangles(&self) -> &Vec<Rectangle> {
        &self.rectangles
    }

    pub fn sort_rectangles(&mut self) {
        self.rectangles
            .sort_by(|a, b| a.area.partial_cmp(&b.area).unwrap());
    }

    pub fn generate_rectangles(&mut self) {
        for i in 0..self.corners.len() {
            for j in 0..self.corners.len() {
                if i != j {
                    let rect = Rectangle::new(self.corners[i].clone(), self.corners[j].clone());

                    if rect.width > 0.0 && rect.height > 0.0 {
                        self.rectangles.push(rect);
                    }
                }
            }
        }
    }

    pub fn add_point(&mut self, line: String) {
        let split = line.split(",").collect::<Vec<&str>>();

        self.corners.push(Point {
            x: split[0].parse::<f64>().unwrap(),
            y: split[1].parse::<f64>().unwrap(),
        });
    }

    pub fn corners(&self) -> &Vec<Point> {
        &self.corners
    }
}
