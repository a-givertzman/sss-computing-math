use super::point::Point;
use super::entity::Entity;

pub struct Line {
    pub start: Point,
    pub terminate: Point,
    pub k: Option<f64>,
}

impl Line {
    fn new(start: Point, terminate: Point) -> Self {
        let k = {
            if start.x - start.y == 0.0 {
                // Vertical line.
                0.0 
            } else if start.y - terminate.y == 0.0 {
                // Horizont line.
                0.0 
            } else {
                (start.x - terminate.x) / (start.y - terminate.y)
            }
        };

        Line { start, terminate, k: Some(k) }
    }

    fn max_point_x(&self) -> f64 {
        if self.start.x > self.terminate.x {
            self.start.x
        } else { self.terminate.x }
    }

    fn min_point_y(&self) -> f64 {
        if self.start.y < self.terminate.y {
            self.start.y
        } else { self.terminate.y }
    }

    fn max_point_y(&self) -> f64 {
        if self.start.y > self.terminate.y {
            self.start.y
        } else { self.terminate.y }
    }

    
}

impl Entity for Line {
    fn crossed(&self, line: &Ray) -> bool {
        if self.k != Some(line.2) {
            if self.max_point_x() > line.0.x
                && line.0.y > self.min_point_y()
                && line.0.y < self.max_point_y() {
                    true
                } else { false }
        } else { false }    
    }

}

pub struct Ray(Point, Point, f64);